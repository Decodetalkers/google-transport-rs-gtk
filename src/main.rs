mod translate;
use gtk::gdk_pixbuf::Pixbuf;
use gtk::prelude::*;
use std::cell::RefCell;
//use gtk::AboutDialog;
//use gtk::{gio, glib};
use gdk::ModifierType;
use gtk::ApplicationWindow;
use gtk::{cairo, gdk, glib, AccelFlags, AccelGroup};
struct Ui{
    label : gtk::Label,
    combo_source : gtk::ComboBoxText,
    combo_target : gtk::ComboBoxText,
}
thread_local!(
    static GLOBAL: RefCell<Option<Ui>> = RefCell::new(None)
);
fn main() {
    //导入resource
    gio::resources_register_include!("compiled.gresource").unwrap();
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
    //window.set_resizable(false);
    window.set_position(gtk::WindowPosition::Center);
    //检查当前目录是否有这个图片，如果有，就加载，没有，就不加载
    if let Ok(icon) = &Pixbuf::from_resource("/ygo/youxie.jpeg") {
        window.set_icon(Some(icon));
    }
    //window.set_icon(Some(&Pixbuf::from_file("./youxie.jpeg").unwrap()));
    set_visual(&window, None);

    window.connect_screen_changed(set_visual);
    window.connect_draw(draw);
    let accel_group = AccelGroup::new();
    window.add_accel_group(&accel_group);
    let (key, modifier) = gtk::accelerator_parse("<Primary>C");

    let v_box = gtk::Box::new(gtk::Orientation::Vertical, 10);
    let label = gtk::Label::new(Some("Here is translate"));
    //设置最大字符传，过了就换行
    label.set_max_width_chars(40);
    label.set_line_wrap(true);

    let copy = gtk::Button::with_mnemonic("Copy");
    let combo = gtk::ComboBoxText::new();
    combo.append_text("zh");
    combo.append_text("en");
    combo.append_text("ja");
    combo.append_text("fr");
    combo.append_text("km");
    combo.set_active(Some(1));
    let combo2 = gtk::ComboBoxText::new();
    combo2.append_text("zh");
    combo2.append_text("en");
    combo2.append_text("ja");
    combo2.append_text("fr");
    combo2.append_text("km");
    combo2.set_active(Some(0));
    //println!("{:?}",combo2.active());
    let button_box = gtk::ButtonBox::new(gtk::Orientation::Horizontal);

    button_box.set_layout(gtk::ButtonBoxStyle::End);
    button_box.pack_start(&combo, false, false, 0);
    button_box.pack_start(&combo2, false, false, 0);
    button_box.pack_start(&copy, false, false, 0);

    v_box.pack_start(&label, true, true, 0);
    v_box.pack_start(&button_box, true, true, 0);
    window.add(&v_box);
    window.set_title("System menu bar");
    window.set_border_width(10);
    window.set_position(gtk::WindowPosition::Center);
    //window.set_default_size(550, 70);

    window.set_app_paintable(true);
    add_actions(&window, &label, &copy, key, &accel_group, modifier);
    window.set_resizable(false);
    //设置位置在左上角
    window.move_(0, 0);
    window.show_all();
    GLOBAL.with(move |global| {
        *global.borrow_mut() = Some(Ui{
            label,
            combo_source:combo,
            combo_target:combo2,
        });
    });
    //listen to the click board;
    let clipboard = gtk::Clipboard::get(&gdk::SELECTION_CLIPBOARD);
    clipboard
        .connect("owner-change", true, |_| {
            let clipbord = gtk::Clipboard::get(&gdk::SELECTION_CLIPBOARD);
            clipbord.request_text(move |_, b| {
                if let Some(word) = b {


                    GLOBAL.with(|global| {
                        if let Some(ref ui) = *global.borrow() {
                            let mut source2 : &str = "en";
                            let mut target2 : &str = "zh";
                            if let Some(source) = ui.combo_source.active(){
                                match source {
                                    0 => source2 = "zh",
                                    1 => source2 = "en",
                                    2 => source2 = "ja",
                                    3 => source2 = "fr",
                                    4 => source2 = "km",
                                    _ => source2 = "en",
                                }
                            }
                            if let Some(target) = ui.combo_target.active(){
                                match target {
                                    0 => target2 = "zh",
                                    1 => target2 = "en",
                                    2 => target2 = "ja",
                                    3 => target2 = "fr",
                                    4 => target2 = "km",
                                    _ => target2 = "zh",
                                }
 
                            }

                            //if ui.source == 0 {
                            //    source = "zh";
                            //}else{
                            //    source = "en";
                            //}
                            //if ui.target == 0{
                            //    target = "zh";
                            //}else {
                            //    target = "en";
                            //}

                            let word = translate::translate(source2,target2,word.to_string());
                            ui.label.set_text(word.as_str());
                        }
                    });

                    //println!("{}",word);
                }
            });

            None
        })
        .map_err(|err| println!("{:?}", err))
        .ok();
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
    label: &gtk::Label,
    button: &gtk::Button,
    key: u32,
    accel_group: &AccelGroup,
    accel_mods: ModifierType,
) {
    button.add_accelerator("clicked", accel_group, key, accel_mods, AccelFlags::VISIBLE);
    button.connect_clicked(glib::clone!(@weak label, @weak window => move |_| {
        window.resize(500, 50);
        let clipbord = gtk::Clipboard::get(&gdk::SELECTION_CLIPBOARD);
        clipbord.set_text(label.text().as_str());
        //clipbord.request_text(move |_,b|{
        //    if let Some(word) = b {
        //        let word = translate::translate(word.to_string());
        //        label.set_text(word.as_str());
        //        //println!("{}",word);
        //    }
        //})
    }));

    //window.is_resizable();
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
    ctx.set_source_rgba(1.0, 1.0, 1.0, 0.7);
    ctx.set_operator(cairo::Operator::Screen);
    ctx.paint().expect("Invalid cairo surface state");
    Inhibit(false)
}
