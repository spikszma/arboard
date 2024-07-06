use arboard::Clipboard;

#[test]
fn get_set_special() {
	env_logger::init();
	let mut ctx = Clipboard::new().unwrap();

	let special_format = "dyn.arboard.pecial.format";

	ctx.set_special(special_format, &[1]).unwrap();
    assert_eq!(ctx.get_special(special_format).unwrap(), vec![1]);

	ctx.set_special(special_format, &[0]).unwrap();
    assert_eq!(ctx.get_special(special_format).unwrap(), vec![0]);
}
