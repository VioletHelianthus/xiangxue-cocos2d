use xiangxue_cocos::CocosBackend;

fn main() {
    xiangxue::run_cli(|w, h| CocosBackend { design_width: w, design_height: h });
}
