use dialoguer::Input;

#[derive(Debug, Clone, interactive_clap_derive::InteractiveClap)]
#[interactive_clap(context = crate::common::SenderContext)]
#[interactive_clap(fn_from_cli = default)]
pub struct Stake {
    pub amount: crate::common::NearBalance,
    #[interactive_clap(named_arg)]
    ///Enter an public key
    pub transactions_signing_public_key: super::transactions_signing::TransactionsSigningAction,
}

impl Stake {
    pub fn from_cli(
        optional_clap_variant: Option<
            <Stake as interactive_clap::ToCli>::CliVariant,
        >,
        context: crate::common::SenderContext,
    ) -> color_eyre::eyre::Result<Self> {
        let amount: crate::common::NearBalance = match &context.connection_config {
            Some(network_connection_config) => {
                let account_balance: crate::common::NearBalance =
                    match crate::common::get_account_state(
                        network_connection_config,
                        context.sender_account_id.clone().into(),
                    )? {
                        Some(account_view) => {
                            crate::common::NearBalance::from_yoctonear(account_view.amount)
                        }
                        None => crate::common::NearBalance::from_yoctonear(0),
                    };
                match optional_clap_variant
                    .clone()
                    .and_then(|clap_variant| clap_variant.amount)
                {
                    Some(cli_amount) => {
                        if cli_amount <= account_balance {
                            cli_amount
                        } else {
                            println!(
                                "You need to enter a value of no more than {}",
                                account_balance
                            );
                            Stake::input_amount(Some(account_balance))?
                        }
                    }
                    None => Stake::input_amount(Some(account_balance))?,
                }
            }
            None => match optional_clap_variant
                .clone()
                .and_then(|clap_variant| clap_variant.amount)
            {
                Some(cli_amount) => cli_amount,
                None => Stake::input_amount(None)?,
            },
        };
        let transactions_signing_public_key = super::transactions_signing::TransactionsSigningAction::from_cli(
            optional_clap_variant.and_then(|clap_variant| match clap_variant.transactions_signing_public_key {
                Some(ClapNamedArgTransactionsSigningActionForStake::TransactionsSigningPublicKey(cli_transactions_signing_public_key)) => Some(cli_transactions_signing_public_key),
                None => None,
            }),
            context
        )?;
        Ok(Self {
            amount,
            transactions_signing_public_key,
        })
    }
}

impl Stake {
    fn input_amount(
        account_balance: Option<crate::common::NearBalance>,
    ) -> color_eyre::eyre::Result<crate::common::NearBalance> {
        match account_balance {
            Some(account_balance) => loop {
                let input_amount: crate::common::NearBalance = Input::new()
                            .with_prompt("How many NEAR Tokens do you want to transfer? (example: 10NEAR or 0.5near or 10000yoctonear)")
                            .with_initial_text(format!("{}", account_balance))
                            .interact_text()
                            .unwrap();
                if input_amount <= account_balance {
                    break Ok(input_amount);
                } else {
                    println!(
                        "You need to enter a value of no more than {}",
                        account_balance
                    )
                }
            }
            None => Ok(Input::new()
                        .with_prompt("How many NEAR Tokens do you want to transfer? (example: 10NEAR or 0.5near or 10000yoctonear)")
                        .interact_text()
                        .unwrap())
        }
    }

    pub async fn process(
        self,
        prepopulated_unsigned_transaction: near_primitives::transaction::Transaction,
        network_connection_config: Option<crate::common::ConnectionConfig>,
    ) -> crate::CliResult {
        self.transactions_signing_public_key
            .process(
                prepopulated_unsigned_transaction,
                network_connection_config,
                self.amount.to_yoctonear(),
            )
            .await
    }
}
