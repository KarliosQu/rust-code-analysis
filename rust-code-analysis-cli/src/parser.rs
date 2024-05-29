use serde::Deserialize;

#[derive(Deserialize, Debug)]

pub struct Doc {
    pub name: String,
    pub start_line: u32,
    pub end_line: u32,
    pub kind: String,
    pub spaces: Vec<Doc>,
    pub metrics: Metrics,
}

#[derive(Deserialize, Debug)]
pub struct Metrics {
    pub nargs: NargsMetricsVal,
    pub nexits: MetricsVal,
    pub congnitive: MetricsVal,
    pub cyclomatic: MetricsVal,
    pub halstead: Halstead,
    pub loc: Loc,
    pub nom: Nom,
    pub mi: Mi,
    #[serde(default)]
    pub abc: Abc,
}

#[derive(Deserialize, Debug, Clone, Copy)]
pub struct MetricsVal {
    pub sum: f32,
    pub average: f32,
    pub min : f32,
    pub max: f32,
}

impl ToString for MetricsVal{
    fn to_string(&self) -> String{
        format!("总和：{}  平均值：{} 最大值：{} 最小值：{}", self.sum, self.average, self.min, self.max)
    }
}

#[derive(Deserialize, Debug, Clone, Copy)]
pub struct NargsMetricsVal{
    pub total: f32,
    pub average: f32,
}

impl ToString for NargsMetricsVal{
    fn to_string(&self) -> String{
        format!("总和：{}  平均值：{}", self.total, self.average)
    }
}

#[derive(Deserialize, Debug, Clone, Copy)]

pub struct Loc{
    pub sloc: f32,
    pub ploc: f32,
    pub lloc: f32,
    pub cloc: f32,
    pub blank: f32,
}

#[derive(Deserialize, Debug)]

pub struct Nom {
    pub functions: f32,
    pub closures: f32,
    pub total: f32,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug, Clone, Copy)]

pub struct Halstead{
    pub n1: f32,
    pub N1: f32,
    pub n2: f32,
    pub N2: f32,
    pub length: f32,
    pub estimated_program_length: f32,
    pub purity_ratio: f32,
    pub vocabulary: f32,
    pub volume: f32,
    pub difficulty: f32,
    pub level: f32,
    pub effort: f32,
    pub time: f32,
    pub bugs: f32,
}

#[derive(Deserialize, Debug, Clone, Copy)]
pub struct Mi {
    pub mi_original: f32,
    pub mi_sei: f32,
    pub mi_visual_studio: f32,
}

#[derive(Deserialize, Debug, Clone, Copy, Default)]
pub struct Abc {
    pub assignments: f32,
    pub branches: f32,
    pub conditions: f32,
    pub magnitude: f32,
}

#[derive(Debug, Clone, Copy)]
pub struct Function<'p>{
    pub name: &'p str,
    pub order: usize,
    pub selected: bool,
    pub option_a: bool,
    pub option_b: bool,
}

#[derive(Debug)]
pub struct Function<'p> {
    pub values: Ves<Function<'p>>,
}

impl<'p> From<Vec<Function<'p>>> for Functions<'p>{
    fn form(v: Vec<Function<'p>>) -> Self{
        Self {values: v}
    }
}

impl<'p> Functions<'p> {
    pub fn to_html_option_script_string(&self, y: &str) -> String{
        let mut res = String::new();
        for item in &self.values{
            res.push_str(&item.to_html_option_script_string(y));
        }
        res
    }

    pub fn to_dropbox_option_string(&self) -> String{
        let mut res = String::new();
        for item in &self.values{
            res.push_str(&item.to_html_option_script_string());
        }
        res
    }
}

impl<'p> Functions<'p> {
    pub fn new(name: &'p str, order: usize) -> Self{
        Self{
            name,
            order,
            selected: false,
            option_a: true,
            option_b: true,
        }
    }

    pub fn to_html_option_script_string(&self, y:&str) -> String{
        format!(
            "{y}.options.add(new Option(\"{}\", \"{}\", {}, {}));\n",
            self.name, self.name, self.option_a, self.option_b,
        )
    }

    pub fn to_dropbox_option_string(&self) -> String{
        format!(
            "<option value={}\"{}\">{}</option>\n"
            if self.selected {
                "selected=\"selected\""
            } else {
                ""
            },
            self.name,
            self.name,
        )
    }
}
