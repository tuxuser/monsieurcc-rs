use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LedColor {
    White,
    Green,
    Red,
    Off,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum NutrientType {
    Joules,
    Calories,
    Protein,
    Carbohydrate,
    Fat
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum StepMode {
    Preparation,
    Kneading,
    Roasting,
    Ramp,
    Wait,
    Scale,
    Turbo,
    Instruction,
    Cooking,
    Steaming,
    End,
}


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RecipeIds {
    pub ids: Vec<u32>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Recipe {
    pub data: Data,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Data {
    pub id: i64,
    pub new: i64,
    pub name: String,
    pub tags: Vec<Tag>,
    pub unit: String,
    pub level: i64,
    #[serde(rename = "yield")]
    pub yield_field: i64,
    pub remove: bool,
    pub updated: String,
    pub version: i64,
    pub download: serde_json::Value,
    pub duration: i64,
    pub language: String,
    #[serde(rename = "originID")]
    pub origin_id: String,
    #[serde(rename = "valid_to")]
    pub valid_to: Option<String>,
    pub image_base: String,
    pub image_name: String,
    pub nutrients: Vec<Nutrient>,
    pub yield_unit: String,
    pub complexity: String,
    #[serde(rename = "valid_from")]
    pub valid_from: Option<String>,
    pub machine_type: String,
    pub version_tags: Option<Vec<serde_json::Value>>,
    pub instructions: Vec<String>,
    pub preparations: Vec<serde_json::Value>,
    pub duration_total: i64,
    pub guided_cooking: GuidedCooking,
    pub scheme_version: i64,
    pub machine_version: i64,
    pub ingredients_bases: Vec<IngredientsBase>,
    pub betarecipetype: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tag {
    pub name: String,
    pub category: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Nutrient {
    #[serde(rename = "type")]
    pub type_field: NutrientType,
    pub unit: String,
    pub amount: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GuidedCooking {
    pub steps: Vec<Step>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Step {
    pub led: Led,
    pub mode: StepMode,
    pub step: i64,
    pub text: String,
    pub measurement: Measurement,
    pub machine_values: MachineValues,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Led {
    pub color: LedColor,
    pub action: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Measurement {
    pub lid: bool,
    pub temp: Option<i64>,
    pub speed: Option<i64>,
    pub weight: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MachineValues {
    pub temp: Option<i64>,
    pub time: Option<i64>,
    pub speed: Option<i64>,
    pub weight: Option<i64>,
    pub reverse: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IngredientsBase {
    pub name: String,
    pub ingredients: Vec<Ingredient>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ingredient {
    pub name: String,
    pub unit: String,
    pub amount: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;
    use crate::tests;

    #[rstest]
    #[case::beta("recipe_all_beta.json", 1)]
    #[case::default("recipe_all_default.json", 1)]
    #[case::live("recipe_all_live.json", 1)]
    fn deserialize_recipes_all(
        #[case] filename: &str,
        #[case] expected_len: usize,
    ) {
        let json = tests::get_testdata(filename)
            .expect("Failed to get testdata");
        
        let res: Vec<Recipe> = serde_json::from_slice(&json)
            .expect("Failed to deserialize");

        assert_eq!(res.len(), expected_len);
    }

    #[rstest]
    #[case("recipe_single_25011.json", 25011)]
    fn deserialize_recipe_single(
        #[case] filename: &str,
        #[case] expected_id: i64,
    ) {
        let json = tests::get_testdata(filename)
            .expect("Failed to get testdata");
        
        let res: Recipe = serde_json::from_slice(&json)
            .expect("Failed to deserialize");

        assert_eq!(res.data.id, expected_id);
    }
}