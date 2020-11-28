use vm;

pub struct BenchmarkModule {}

impl vm::Module for BenchmarkModule {
    fn init() {
        let command = vm::data::Command::empty(vm::commands::gapi::SET_COLOR_PIPELINE);
        vm::push_command(command, vm::Source::GAPI);
    }

    fn drop() {}

    fn step() {}

    fn render() {
        let points = [
            vm::data::Vec2f::new(0.0, 0.0),
            vm::data::Vec2f::new(100.0, 0.0),
            vm::data::Vec2f::new(100.0, 100.0),
            vm::data::Vec2f::new(0.0, 100.0),
        ];

        let command_payload = unsafe { vm::data::CommandPayload::new(&points) };
        let command = vm::data::Command::new(vm::commands::gapi::DRAW_PATH, command_payload);

        vm::push_command(command, vm::Source::GAPI);
    }
}
