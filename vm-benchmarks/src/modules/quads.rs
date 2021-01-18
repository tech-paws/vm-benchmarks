use vm::{
    commands,
    commands_bus::Source,
    data::{Command, BytesBuffer, Vec2f},
    module::{Module, ModuleState, CLIENT_ID},
};

pub struct QuadsModule {}

impl QuadsModule {
    pub fn new() -> Self {
        QuadsModule {}
    }
}

impl Module for QuadsModule {
    fn init(&mut self, state: &mut ModuleState) {
        let command = Command::empty(commands::gapi::SET_COLOR_PIPELINE);
        let commands_bus = &state.commands_bus;
        commands_bus.push_command(CLIENT_ID, command, Source::GAPI);
    }

    fn shutdown(&mut self, _: &mut ModuleState) {}

    fn step(&mut self, _: &mut ModuleState) {}

    fn render(&mut self, state: &mut ModuleState) {
        let commands_bus = &state.commands_bus;

        let points = [
            Vec2f::new(0.0, 0.0),
            Vec2f::new(100.0, 0.0),
            Vec2f::new(100.0, 100.0),
            Vec2f::new(0.0, 100.0),
        ];

        let command_payload = BytesBuffer::new(&points);
        let command = Command::new(commands::gapi::DRAW_QUADS, command_payload);

        commands_bus.push_command(CLIENT_ID, command, Source::GAPI);
    }
}
