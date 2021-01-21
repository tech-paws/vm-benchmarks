use vm::{
    commands,
    commands::Source,
    data::{BytesBuffer, Command, Vec2f, Vec4f},
    module::{Module, ModuleState, CLIENT_ID},
};

pub struct QuadsModule {}

impl QuadsModule {
    pub fn new() -> Self {
        QuadsModule {}
    }
}

impl Module for QuadsModule {
    fn init(&mut self, _: &mut ModuleState) {}

    fn shutdown(&mut self, _: &mut ModuleState) {}

    fn step(&mut self, _: &mut ModuleState) {}

    fn render(&mut self, state: &mut ModuleState) {
        let commands_bus = &state.commands_bus;

        let color = Vec4f::new(1.0, 0.0, 0.0, 1.0);
        let command_payload = BytesBuffer::new(&[color]);
        let command = Command::new(commands::gapi::SET_COLOR_PIPELINE, command_payload);

        commands_bus.push_command(CLIENT_ID, command, Source::GAPI);

        let args = [
            // Position
            Vec2f::new(100.0, 100.0),
            // Size
            Vec2f::new(100.0, 100.0),
        ];

        let command_payload = BytesBuffer::new(&args);
        let command = Command::new(commands::gapi::DRAW_QUADS, command_payload);

        commands_bus.push_command(CLIENT_ID, command, Source::GAPI);
    }
}
