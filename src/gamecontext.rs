//use vulkano::app_info_from_cargo_toml;

use crate::util::log_info;

pub struct InitError 
{
        pub print_error_to: std::fs::File,  // give log file back to `main`
        pub error_str: String,
}
pub struct GameContext 
{
        _pref_path: String,
        _log_file: std::fs::File,
        _sdlc: sdl2::Sdl,
        _sdl_vss: sdl2::VideoSubsystem,
        _game_window: sdl2::video::Window/*,
        _vkinst: vulkano::instance::Instance*/
}
impl GameContext 
{
        // game context "constructor"
        pub fn new(log_file: std::fs::File, pref_path: String) 
        -> Result<GameContext, InitError> 
        {
                // print start date and time
                let datetime_str = format!(
                        "INIT {}", 
                        chrono::Local::now().to_rfc3339()
                );
                log_info(&log_file, &datetime_str);

                // get command line arguments
                // let args: Vec<String> = std::env::args().collect();

                // initialize SDL2
                let sdl_context;
                match sdl2::init() {
                        Ok(sc) => sdl_context = sc,
                        Err(e) => return Err(
                                InitError{ 
                                        print_error_to: log_file,  
                                        error_str: e 
                                }
                        )
                }
                
                // initialize SDL2 video subsystem
                let sdl_vss;
                match sdl_context.video() {
                        Ok(vss) => sdl_vss = vss,
                        Err(e) => return Err(
                                InitError{ 
                                        print_error_to: log_file, 
                                        error_str: e 
                                }
                        )
                }

                // create window
                let wnd_result = sdl_vss.window("MithrilEngine", 1280, 720)
                        .position_centered()
                        .vulkan()
                        .build();
                let gwnd;
                match wnd_result {
                        Ok(w) => gwnd = w,
                        Err(e) => return Err(
                                InitError{ 
                                        print_error_to: log_file, 
                                        error_str: e.to_string() 
                                }
                        )
                }

                // create Vulkan instance
                /*let app_info = app_info_from_cargo_toml!();
                app_info.engine_name = "MithrilEngine";
                let vkinst_result = vulkano::instance::Instance::new();*/

                Ok(GameContext { 
                        _pref_path: pref_path,
                        _log_file: log_file,
                        _sdlc: sdl_context,
                        _sdl_vss: sdl_vss,
                        _game_window: gwnd
                })
        }

        pub fn render_loop(&self) 
        {
                match self._render_loop_inner() {
                        Ok(()) => (),
                        Err(e) => {
                                self._render_loop_error(e);
                        }
                }
        }

        pub fn print_log(&self, s: &str) 
        {
                log_info(&self._log_file, s);
        }

        fn _render_loop_error(&self, e: Box<dyn std::error::Error>) 
        {
                self.print_log(&format!("ERROR: {}", &e.to_string()));
                match msgbox::create(
                        "MithrilEngine Error", 
                        &e.to_string(), 
                        msgbox::common::IconType::Error
                ) {
                        Ok(r) => r,
                        Err(mbe) => {
                                let mbe_str = [
                                        "Failed to create error message box: ", 
                                        &mbe.to_string()
                                ].concat();
                                self.print_log(&mbe_str);
                        }
                }
        }

        fn _render_loop_inner(&self) -> Result<(), Box<dyn std::error::Error>> 
        {
                // wait for 2 seconds
                std::thread::sleep(std::time::Duration::from_millis(2000));

                Ok(())
        }
}