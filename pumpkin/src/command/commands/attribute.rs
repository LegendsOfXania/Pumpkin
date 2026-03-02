use pumpkin_util::text::TextComponent;
use crate::command::args::{Arg, ConsumedArgs, FindArgDefaultName, FindArg};
use crate::command::args::entity::EntityArgumentConsumer;
use crate::command::args::resource::attribute::AttributeArgumentConsumer;
use crate::command::{CommandExecutor, CommandResult, CommandSender};
use crate::command::args::bounded_num::BoundedNumArgumentConsumer;
use crate::command::dispatcher::CommandError::InvalidConsumption;
use crate::command::tree::builder::{argument, argument_default_name, literal};
use crate::command::tree::CommandTree;
use crate::entity::EntityBase;

const NAMES: [&str; 1] = ["attribute"];

const DESCRIPTION: &str = "Query and modify an entity's attribute";

const ARG_TARGET: &str = "target";
const ARG_SCALE: &str = "scale";

struct GetExecutor;

impl CommandExecutor for GetExecutor {
    fn execute<'a>(
        &'a self,
        sender: &'a CommandSender,
        _server: &'a crate::server::Server,
        args: &'a ConsumedArgs<'a>,
    ) -> CommandResult<'a> {
        Box::pin(async move {
            let Arg::Entity(target) = args
                .get(ARG_TARGET)
                .ok_or(InvalidConsumption(Some(ARG_TARGET.into())))?
            else {
                return Err(InvalidConsumption(Some(ARG_TARGET.into())));
            };

            let scale = get_scale(args);

            let target = target.get_living_entity().unwrap();
            let attribute = AttributeArgumentConsumer.find_arg_default_name(args)?;

            let attribute_name = attribute.name
                .strip_prefix("minecraft:")
                .unwrap_or(&attribute.name)
                .to_string();
            let attribute_value = target.get_attribute_value(attribute);

            sender.send_message(TextComponent::translate(
                "commands.attribute.value.get.success",
                [
                    TextComponent::text(attribute_name),
                    target.get_display_name().await,
                    TextComponent::text(attribute_value.to_string()),
                ],
            )).await;

            Ok((attribute_value * scale) as i32)
        })
    }
}

const fn scale_consumer() -> BoundedNumArgumentConsumer<f64> {
    BoundedNumArgumentConsumer::new().name(ARG_SCALE)
}

fn get_scale(args: &ConsumedArgs) -> f64 {
    match BoundedNumArgumentConsumer::<f64>::find_arg(args, ARG_SCALE) {
        Ok(Ok(scale)) => scale,
        _ => 1.0,
    }
}

pub fn init_command_tree() -> CommandTree {
    CommandTree::new(NAMES, DESCRIPTION).then(
        argument(ARG_TARGET, EntityArgumentConsumer).then(
            argument_default_name(AttributeArgumentConsumer).then(
                literal("get").execute(GetExecutor).then(
                    argument_default_name(scale_consumer()).execute(GetExecutor)
                )
            )
        )
    )
}