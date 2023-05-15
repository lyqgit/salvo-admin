use captcha::Captcha;
use captcha::filters::Noise;

#[warn(dead_code)]
pub fn create_captcha()->(String,Option<String>){
    let mut captcha_obj = Captcha::new();
    captcha_obj.add_chars(5)
        .apply_filter(Noise::new(0.1))
        .view(300, 120);
        // .save(Path::new("static/captcha.png"))
        // .expect("save failed");
    (captcha_obj.chars_as_string(),captcha_obj.as_base64())
}