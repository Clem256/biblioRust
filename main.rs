use eframe::egui;
use serde::{Serialize, Deserialize};
use serde_json;
use eframe::egui::Color32;


fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Gestion de la bibliothèque",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    )
}

#[derive(Serialize, Deserialize, Debug)]
struct MyApp {
    name: String,
    resume : String,
    types: String,
    number : i32,
    etat_parution : String,
    author: String,
    resell: bool,
    display_text: String,
    books: Vec<String>,
}

impl Default for MyApp {  
    fn default() -> Self {
        let db = sled::open("books").expect("db Error"); 
        let mut books = Vec::new();

        if let Ok(Some(bytes)) = db.get("books") { 
            if let Ok(saved_books) = serde_json::from_slice::<Vec<String>>(&bytes) { 
                books = saved_books;
            }
        }

        Self {
            name: String::new(),
            resume : String::new(),
            types: String::new(), 
            number : 0,
            etat_parution : String::new(),
            author: String::new(),
            resell: false,
            display_text: String::new(),
            books,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
         let my_frame = egui::containers::Frame {
            inner_margin: egui::epaint::Margin { left: 10., right: 10., top: 10., bottom: 10., },
            outer_margin: egui::epaint::Margin { left: 10., right: 10., top: 10., bottom: 10., },
            rounding: egui::Rounding { nw: 1.0, ne: 1.0, sw: 1.0, se: 1.0, },
            shadow: eframe::epaint::Shadow { color: Color32::WHITE, offset: egui::Vec2 { x: 0., y: 0. }, blur: 0., spread: 0., },
            fill: Color32::BLACK,
            stroke: egui::Stroke::new(1.0, Color32::BLACK),
        };
        egui::CentralPanel::default().frame(my_frame).show(ctx, |ui| {
            ui.heading("Books management");

            ui.label("Name :");
            ui.text_edit_singleline(&mut self.name);

            ui.label("Resume :");
            ui.text_edit_multiline(&mut self.resume);
            
            ui.label("Type (Manga/LN/Comics/Novel) :");
            ui.text_edit_singleline(&mut self.types);

            ui.label("Number of books :");
            ui.add(egui::DragValue::new(&mut self.number).clamp_range(1..=100));

            ui.label("Publication status (in progress/completed) :");
            ui.text_edit_singleline(&mut self.etat_parution);

            ui.label("Author ");
            ui.text_edit_singleline(&mut self.author);

            ui.label("Resell :");
            ui.checkbox(&mut self.resell, "Check if resell");

            if ui.button("Add").clicked() {
                 if self.resell {
                     self.display_text = format!(
                         "Name: {}, Type: {}, Book number : {} , Publication status : {} , Author : {}, Resell",
                         self.name, self.types,self.number ,self.etat_parution ,  self.author
                     );
                 }
                else {
                    self.display_text = format!(
                         "Name: {}, Type: {}, Book number : {} , Publication status : {} , Author : {}, No Resell",
                        self.name, self.types,self.number ,self.etat_parution ,  self.author
                    );
                }
                self.books.push(self.display_text.clone());
                if let Ok(db) = sled::open("books") {
                    if let Ok(book_json) = serde_json::to_vec(&self.books) {
                        let _ = db.insert("books", book_json);
                        let _ = db.flush();
                    }
                }
            }
            for i in 0.. self.books.len() { 
                ui.horizontal(|ui| {
                    ui.label(&self.books[i]);
                    if ui.button("Delete").clicked() {
                        let removed = self.books.remove(i);
                        if let Ok(db) = sled::open("books") {
                            if let Ok(book_json) = serde_json::to_vec(&self.books) {
                                let _ = db.insert("books", book_json);
                                let _ = db.flush();
                            }
                        }
                    }
                });
            }

        });
    }
}
