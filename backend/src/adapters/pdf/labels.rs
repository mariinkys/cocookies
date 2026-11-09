//! Localised labels of the PDF templates.

use crate::entities::recipe::PdfTemplate;

pub struct Labels {
    pub language: &'static str,
    pub servings: &'static str,
    pub prep: &'static str,
    pub cook: &'static str,
    pub minutes: &'static str,
    pub ingredients: &'static str,
    pub instructions: &'static str,
    pub no_ingredients: &'static str,
    pub no_steps: &'static str,
    pub step: &'static str,
    pub created: &'static str,
    pub made_with: &'static str,
    pub updated: &'static str,
}

const EN: Labels = Labels {
    language: "en",
    servings: "servings",
    prep: "min prep",
    cook: "min cook",
    minutes: "min",
    ingredients: "Ingredients",
    instructions: "Instructions",
    no_ingredients: "No ingredients listed.",
    no_steps: "No steps listed.",
    step: "Step",
    created: "Created:",
    made_with: "Made with Cocookies",
    updated: "Updated At:",
};

const ES: Labels = Labels {
    language: "es",
    servings: "raciones",
    prep: "min prep",
    cook: "min cocción",
    minutes: "min",
    ingredients: "Ingredientes",
    instructions: "Instrucciones",
    no_ingredients: "No hay ingredientes.",
    no_steps: "No hay pasos.",
    step: "Paso",
    created: "Creada:",
    made_with: "Hecho con Cocookies",
    updated: "Actualizada:",
};

pub fn labels(template: PdfTemplate) -> &'static Labels {
    match template {
        PdfTemplate::DefaultEnglish => &EN,
        PdfTemplate::DefaultSpanish => &ES,
    }
}
