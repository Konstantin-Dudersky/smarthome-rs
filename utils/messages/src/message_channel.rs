use rsiot::message::IMessageChannel;

#[derive(Clone, Debug)]
pub enum MessageChannel {
    Output,
}

impl IMessageChannel for MessageChannel {}
