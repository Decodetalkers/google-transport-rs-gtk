mod translate;
use gtk::gdk_pixbuf::Pixbuf;
use gtk::prelude::*;
//use gtk::AboutDialog;
//use gtk::{gio, glib};
use gdk::ModifierType;
use gtk::{cairo, gdk,glib,AccelFlags,AccelGroup};
use gtk::ApplicationWindow;
fn main() {
    let application = gtk::Application::new(
        Some("com.github.gtk-rs.examples.menu_bar_system"),
        Default::default(),
    );

    //直接用回调函数就得了
    //application.connect_startup(add_accelerators);
    application.connect_activate(build_ui);

    application.run();
}

fn build_ui(application: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(application);
    //去除系统装饰
    window.set_decorated(false);
    window.set_keep_above(true);
    //window.set_allocation(
    //    &gtk::Rectangle{
    //        x:0,
    //        y:0,
    //        width:50,
    //        height:30,

    //});
    //设置是否可变大小
    window.set_resizable(false);
    window.set_position(gtk::WindowPosition::Center);
    //检查当前目录是否有这个图片，如果有，就加载，没有，就不加载
    if let Ok(icon) =  &Pixbuf::from_file("./youxie.jpeg") {
        window.set_icon(Some(icon));
    }
    //window.set_icon(Some(&Pixbuf::from_file("./youxie.jpeg").unwrap()));
    set_visual(&window, None);

    window.connect_screen_changed(set_visual);
    window.connect_draw(draw);
    let accel_group = AccelGroup::new();
    window.add_accel_group(&accel_group);
    let (key, modifier) = gtk::accelerator_parse("<Primary>Q");

    let v_box = gtk::Box::new(gtk::Orientation::Vertical, 10);
    let label = gtk::Label::new(Some("Here is translate"));
    
    let translate = gtk::Button::with_mnemonic("Translate");

    let button_box = gtk::ButtonBox::new(gtk::Orientation::Horizontal);

    button_box.set_layout(gtk::ButtonBoxStyle::End);
    button_box.pack_start(&translate,false,false,0);
    v_box.pack_start(&label, true, true, 0);
    v_box.pack_start(&button_box, true, true, 0);
    window.add(&v_box);
    window.set_title("System menu bar");
    window.set_border_width(10);
    window.set_position(gtk::WindowPosition::Center);
    window.set_default_size(350, 70);
    window.set_app_paintable(true);
    add_actions(&window,&label,&translate,key,&accel_group,modifier);
    window.show_all();
}
//fn change_the_label(
//    label : &gtk::Label,
//){
//        let clipbord = gtk::Clipboard::get(&gdk::SELECTION_CLIPBOARD);
//        clipbord.request_text(glib::clone!(@weak label => move |_,b|{
//            match b{
//                Some(word) =>{
//                    label.set_text(word);
//                    println!("{}",word);
//                }
//                None=>{
//                    println!("None");
//                }
//            }
//        }));
//
//}
fn add_actions(
    window: &gtk::ApplicationWindow,
    label : &gtk::Label,
    button: &gtk::Button,
    key : u32,
    accel_group :&AccelGroup,
    accel_mods: ModifierType,
) {
    button.add_accelerator("clicked", accel_group, key, accel_mods, AccelFlags::VISIBLE);
    button.connect_clicked(glib::clone!(@weak label => move |_| {
        let clipbord = gtk::Clipboard::get(&gdk::SELECTION_CLIPBOARD);
        clipbord.request_text(move |_,b|{
            match b{
                Some(word) =>{
                    let word = translate::translate(word.to_string());
                    label.set_text(word.as_str());

                    println!("{}",word);
                }
                None=>{
                    println!("None");
                }
            }
        })
    }));
    window.is_resizable();
    window.resize(100, 100);
    window.set_resizable(false);
}
fn set_visual(window: &ApplicationWindow, _screen: Option<&gdk::Screen>) {
    if let Some(screen) = window.screen() {
        if let Some(ref visual) = screen.rgba_visual() {
            window.set_visual(Some(visual)); // crucial for transparency
        }
    }
}

fn draw(_window: &ApplicationWindow, ctx: &cairo::Context) -> Inhibit {
    // crucial for transparency
    // color set
    ctx.set_source_rgba(0.2, 0.0, 1.0, 0.5);
    ctx.set_operator(cairo::Operator::Screen);
    ctx.paint().expect("Invalid cairo surface state");
    Inhibit(false)
}
