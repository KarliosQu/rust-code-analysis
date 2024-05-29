use anyhow::Result;
use serde_yaml::from_str;
use std::collections::HashMap;

use crate::parser::*;

pub fn write_html (space: String) -> String{
    let mut file_name: Vec<Funcion> = vec![];
    let docs = read_yaml(space).unwrap();

    let mut relation: HashMap<String, Functions> = HashMap::new();

    for file_cir in 0..docs.len() {
        let mut function_name: Vec<Function> = vec![];
        file_name.push(Function::new(docs[file_cir].name.as_str(), file_cir));

        function_name.push(Function::new(
            docs[file_cir].name.as_str(),
            0,
        ));

        for function_cir in 0..docs[file_cir].spaces.len(){
            function_name.push(Function::new(
                docs[file_cir].spaces[function_cir].name.as_str(),
                function_cir + 1,
            ));
        }

        relation.insert(docs[file_cir].name.clone(), function_name.clone().into());
    }

    let mut file_list = String::new();

    for file_cir in 0..docs.len() {
        let mut function_list = String::new();

        function_list.push_str(&format!(
            "new Function (\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\"),",
            docs[file_cir].name.replace('\\', "\\\\"),
            docs[file_cir].start_line,
            docs[file_cir].end_line,
            docs[file_cir].metrics.nargs.to_string(),
            docs[file_cir].metrics.nexits.to_string(),
            docs[file_cir].metrics.congnitive.to_string(),
            docs[file_cir].metrics.cyclomatic.to_string(),
            docs[file_cir].metrics.halstead.n1,
            docs[file_cir].metrics.halstead.N1,
            docs[file_cir].metrics.halstead.n2,
            docs[file_cir].metrics.halstead.N2,
            docs[file_cir].metrics.halstead.length,
            docs[file_cir].metrics.halstead.estimated_program_length,
            docs[file_cir].metrics.halstead.purity_ratio,
            docs[file_cir].metrics.halstead.vocabulary,
            docs[file_cir].metrics.halstead.volume,
            docs[file_cir].metrics.halstead.difficulty,
            docs[file_cir].metrics.halstead.level,
            docs[file_cir].metrics.halstead.effort,
            docs[file_cir].metrics.halstead.time,
            docs[file_cir].metrics.halstead.bugs,
            docs[file_cir].metrics.loc.sloc,
            docs[file_cir].metrics.loc.ploc,
            docs[file_cir].metrics.loc.lloc,
            docs[file_cir].metrics.loc.cloc,
            docs[file_cir].metrics.loc.blank,
            docs[file_cir].metrics.nom.functions,
            docs[file_cir].metrics.nom.closures,
            docs[file_cir].metrics.nom.total,
            docs[file_cir].metrics.mi.mi_original,
            docs[file_cir].metrics.mi.mi_sei,
            docs[file_cir].metrics.mi.mi_visual_studio,
            docs[file_cir].metrics.abc.assignments,
            docs[file_cir].metrics.abc.branches,
            docs[file_cir].metrics.abc.conditions,
            docs[file_cir].metrics.abc.magnitude,
        ));
        for function_cir in 0..docs[file_cir].spaces.len() {
            function_list.push_str(&format!(
                "new Function (\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",)",
                docs[file_cir].spaces[function_cir].spaces[function_cir].name.replace('\\', "\\\\"),
                docs[file_cir].spaces[function_cir].start_line,
                docs[file_cir].spaces[function_cir].end_line,
                docs[file_cir].spaces[function_cir].metrics.nargs.to_string(),
                docs[file_cir].spaces[function_cir].metrics.nexits.to_string(),
                docs[file_cir].spaces[function_cir].metrics.congnitive.to_string(),
                docs[file_cir].spaces[function_cir].metrics.cyclomatic.to_string(),
                docs[file_cir].spaces[function_cir].metrics.halstead.n1,
                docs[file_cir].spaces[function_cir].metrics.halstead.N1,
                docs[file_cir].spaces[function_cir].metrics.halstead.n2,
                docs[file_cir].spaces[function_cir].metrics.halstead.N2,
                docs[file_cir].spaces[function_cir].metrics.halstead.length,
                docs[file_cir].spaces[function_cir].metrics.halstead.estimated_program_length,
                docs[file_cir].spaces[function_cir].metrics.halstead.purity_ratio,
                docs[file_cir].spaces[function_cir].metrics.halstead.vocabulary,
                docs[file_cir].spaces[function_cir].metrics.halstead.volume,
                docs[file_cir].spaces[function_cir].metrics.halstead.difficulty,
                docs[file_cir].spaces[function_cir].metrics.halstead.level,
                docs[file_cir].spaces[function_cir].metrics.halstead.effort,
                docs[file_cir].spaces[function_cir].metrics.halstead.time,
                docs[file_cir].spaces[function_cir].metrics.halstead.bugs,
                docs[file_cir].spaces[function_cir].metrics.loc.sloc,
                docs[file_cir].spaces[function_cir].metrics.loc.ploc,
                docs[file_cir].spaces[function_cir].metrics.loc.lloc,
                docs[file_cir].spaces[function_cir].metrics.loc.cloc,
                docs[file_cir].spaces[function_cir].metrics.loc.blank,
                docs[file_cir].spaces[function_cir].metrics.nom.functions,
                docs[file_cir].spaces[function_cir].metrics.nom.closures,
                docs[file_cir].spaces[function_cir].metrics.nom.total,
                docs[file_cir].spaces[function_cir].metrics.mi.mi_original,
                docs[file_cir].spaces[function_cir].metrics.mi.mi_sei,
                docs[file_cir].spaces[function_cir].metrics.mi.mi_visual_studio,
                docs[file_cir].spaces[function_cir].metrics.abc.assignments,
                docs[file_cir].spaces[function_cir].metrics.abc.branches,
                docs[file_cir].spaces[function_cir].metrics.abc.conditions,
                docs[file_cir].spaces[function_cir].metrics.abc.magnitude,
            ))
        }
        file_list.push_str(&format!(
            "list[\"{}\"]=[{}]\n",
            docs[file_cir].name.replace('\\', "\\\\"),
            function_list
        ));
    }

    let list_a_items = Functions::from(file_name.clone());
    let list_b_items = relation.get(file_name[0].name).unwrap();

    let mut sss = String::new();
    for i in 0..relation.keys().len() {
        sss.push_str(&format!(
            "if(x.selectedIndex == {i})
            {{
                {}
            }}\n",
            relation
                .get(file_name[i].name)
                .unwrap()
                .to_html_option_script_string("y")
                .replace('\\', "\\\\")
        ))
    }

    let template = format!(
        "
        <!DOCTYPE html>
        <html>
        <body>
        <label for=\"Name\">Files</label>
        <select name=\"Files\" id=\"first\" onChange=\"change()\">
            {}
        </select>

        <select name=\"Functions\" id=\"second\">
            {}
        </select>
        <input type=\"submit\" value=\"Submit\" onclick=\"metrics()\"/>

        目前文件/函数：<span id = \"NAME\">尚未选中</span>

        <script>
        function change()
        {{
            var x = document.getElementById(\"first\");
            var y = document.getElementById(\"second\");
            y.options.length = 0;

            {}

        }}

        </script>
        </body>
        </html>

        <table border=\"1\" cellspacing=\"0\" cellpadding=\"0\" id = \"table \">

            <tr>
                <th>度量指标</th>
                <th>英文原名</th>
                <th>中文译名</th>
                <th>度量值</th>
                <th>度量解释</th>
            </tr>
            <tr>
                <td colspan=\"1\" rowspan=\"6\">基本信息</td>
                <td>start_line</td>
                <td>起始行</td>
                <td>0</td>
                <td>函数/文件的起始行</td>
            </tr>
            <tr>
                <td>end_line</td>
                <td>终止行</td>
                <td>0</td>
                <td>函数/文件的终止行</td>
            </tr>
            <tr>
                <td>nargs</td>
                <td>函数参数</td>
                <td>总和：0 平均值：0</td>
                <td>计算函数/方法的参数数量</td>
            </tr>
            <tr>
                <td>nexits</td>
                <td>离开点</td>
                <td>总和：0 平均值：0 最大值：0 最小值：0</td>
                <td>计算函数/方法可能的退出点数量</td>
            </tr>
            <tr>
                <td>cognitive</td>
                <td>认知复杂度</td>
                <td>总和：0 平均值：0 最大值：0 最小值：0</td>
                <td>辜负按代码被理解的复杂程度，可等同于代码的理解成本。</td>
            </tr>
            <tr>
                <td>cognitive</td>
                <td>认知复杂度</td>
                <td>总和：0 平均值：0 最大值：0 最小值：0</td>
                <td>辜负按代码被理解的复杂程度，可等同于代码的理解成本。</td>
            </tr>
            <tr>
                <td>cyclomatic</td>
                <td>圈复杂度</td>
                <td>总和：0 平均值：0 最大值：0 最小值：0</td>
                <td>用于衡量一个程序模块所包含的判定结构的复杂程度。</td>
            </tr>
            <tr>
                <td colspan=\"1\" rowspan=\"14\">Halstead复杂度</td>
                <td>n1</td>
                <td></td>
                <td>0</td>
                <td>霍尔斯特德复杂度下的参数 n1</td>
            </tr>
            <tr>
                <td>n1</td>
                <td></td>
                <td>0</td>
                <td>霍尔斯特德复杂度下的参数 N1</td>
            </tr>
            <tr>
                <td>n1</td>
                <td></td>
                <td>0</td>
                <td>霍尔斯特德复杂度下的参数 n2</td>
            </tr>
            <tr>
                <td>n1</td>
                <td></td>
                <td>0</td>
                <td>霍尔斯特德复杂度下的参数 N2</td>
            </tr>
            <tr>
                <td>length</td>
                <td>程序长度</td>
                <td>0</td>
                <td>霍尔斯特德复杂度下计算程序长度 N2</td>
            </tr>
            <tr>
                <td>estimated_program_length</td>
                <td>预期程序长度</td>
                <td>0</td>
                <td>霍尔斯特德复杂度下预期的程序长度 N2</td>
            </tr>
            <tr>
                <td>purity_ratio</td>
                <td>纯度比</td>
                <td>0</td>
                <td>霍尔斯特德复杂度下计算纯度比</td>
            </tr>
            <tr>
                <td>vocabulary</td>
                <td>词汇表</td>
                <td>0</td>
                <td>霍尔斯特德复杂度下计算词汇表长度</td>
            </tr>
            <tr>
                <td>volume</td>
                <td>程序体积</td>
                <td>0</td>
                <td>霍尔斯特德复杂度下计算程序体积</td>
            </tr>
            <tr>
                <td>difficulty</td>
                <td>程序复杂度</td>
                <td>0</td>
                <td>霍尔斯特德复杂度下计算的程序复杂度</td>
            </tr>
            <tr>
                <td>level</td>
                <td>程序效率</td>
                <td>0</td>
                <td>霍尔斯特德复杂度下计算程序效率</td>
            </tr>
            <tr>
                <td>effort</td>
                <td>编程工作量</td>
                <td>0</td>
                <td>霍尔斯特德复杂度下计算编程工作量</td>
            </tr>
            <tr>
                <td>time</td>
                <td>编程时间</td>
                <td>0</td>
                <td>霍尔斯特德复杂度下计算编程时间</td>
            </tr>
            <tr>
                <td>bugs</td>
                <td>错误预期值</td>
                <td>0</td>
                <td>霍尔斯特德复杂度下计算错误预期值</td>
            </tr>
            <tr>
                <td colspan=\"1\" rowspan=\"5\">代码行统计信息</td>
                <td>sloc</td>
                <td>代码行数</td>
                <td>0</td>
                <td>指一个统计对象的总代码行数</td>
            </tr>
            <tr>
                <td>ploc</td>
                <td>物理行数</td>
                <td>0</td>
                <td>指一个统计对象的物理代码行数</td>
            </tr>
            <tr>
                <td>lloc</td>
                <td>逻辑行数</td>
                <td>0</td>
                <td>指一个统计对象的逻辑代码行数</td>
            </tr>
            <tr>
                <td>cloc</td>
                <td>注释行数</td>
                <td>0</td>
                <td>指一个统计对象的注释代码行数</td>
            </tr>
            <tr>
                <td>sloc</td>
                <td>代码行数</td>
                <td>0</td>
                <td>指一个统计对象的空白代码行数</td>
            </tr>
            <tr>
                <td colspan=\"1\" rowspan=\"3\">函数统计</td>
                <td>functions</td>
                <td>函数数量</td>
                <td>0</td>
                <td>指一个统计对象的函数数量</td>
            </tr>
            <tr>
                <td>closures</td>
                <td>闭包数量</td>
                <td>0</td>
                <td>指一个统计对象的闭包数量</td>
            </tr>
            <tr>
                <td>total</td>
                <td>总计</td>
                <td>0</td>
                <td>指一个统计对象的函数与闭包数目合计值</td>
            </tr>
            <tr>
                <td colspan=\"1\" rowspan=\"3\">MI可维护性指数</td>
                <td>mi_original</td>
                <td>可维护性指数</td>
                <td>0</td>
                <td>MI可维护性指数（正常计算）</td>
            </tr>
            <tr>
                <td>mi_original</td>
                <td>可维护性指数</td>
                <td>0</td>
                <td>MI可维护性指数（使用SEI标准进行计算）</td>
            </tr>
            <tr>
                <td>mi_original</td>
                <td>可维护性指数</td>
                <td>0</td>
                <td>MI可维护性指数（使用Visual Studio标准进行计算）</td>
            </tr>
            <tr>
                <td colspan=\"1\" rowspan=\"4\">ABC度量指数</td>
                <td>assignments</td>
                <td>任务数</td>
                <td>0</td>
                <td>程序任务数</td>
            </tr>
            <tr>
                <td>branch</td>
                <td>分支数</td>
                <td>0</td>
                <td>程序分支数</td>
            </tr>
            <tr>
                <td>condition</td>
                <td>条件数</td>
                <td>0</td>
                <td>程序条件数</td>
            </tr>
            <tr>
                <td>magnitude</td>
                <td>量级</td>
                <td>0</td>
                <td>程序量级</td>
            </tr>
        </table>

        <script>
        class Function{{
            constructor(
                name,start_line,end_line,
                nargs,nexits,cognitive,cyclomatic,
                n1,N1,n2,N2,length,estimated_program_length,purity_ratio,vocabulary,volume,difficulty,level,effort,time,bugs,
                sloc,ploc,lloc,cloc,blank,
                functions,closures,total,
                mi_original,mi_sei,mi_visual_studio,
                assignments,branches,conditions,magnitude){{
                    this.name = name
                    this.start_line = start_line
                    this.end_line = end_line
                    this.nargs = nargs
                    this.nexits = nexits
                    this.cognitive = cognitive
                    this.cyclomatic = cyclomatic
                    this.n1 = n1
                    this.N1 = N1
                    this.n2 = n2
                    this.N2 = N2
                    this.length = length
                    this.estimated_program_length = estimated_program_length
                    this.purity_radio = purity_radio
                    this.vocabulary = vocabulary
                    this.volume = volume
                    this.difficulty = difficulty
                    this.level = level
                    this.effort = effort
                    this.time = time
                    this.bugs = bugs
                    this.sloc = sloc
                    this.ploc = ploc
                    this.lloc = lloc
                    this.cloc = cloc
                    this.blank = blank
                    this.functions = functions
                    this.closures = closures
                    this.total = total
                    this.mi_original = mi_original
                    this.mi_sei = mi_sei
                    this.mi_visual_studio = mi_visual_studio
                    this.assignments = assignments
                    this.branches = branches
                    this.conditions = conditions
                    this.magnitude = magnitude
                }}
        }}

        var list = {{}};

        {}

        function metrics()
        {{
            var File = document.getElementById(\"first\").value;
            var Function = document.getElementById(\"second\").selectedIndex;
            var Table = document.getElementById(\"table\");

            document.getElementById(\"NAME\").innerHTML = list[File][Function].name;

            Table.rows[1].cells[3].innerHTML = list[File][Function].
            Table.rows[2].cells[2].innerHTML = list[File][Function].
            Table.rows[3].cells[2].innerHTML = list[File][Function].
            Table.rows[4].cells[2].innerHTML = list[File][Function].
            Table.rows[5].cells[2].innerHTML = list[File][Function].
            Table.rows[6].cells[2].innerHTML = list[File][Function].
            Table.rows[7].cells[3].innerHTML = list[File][Function].
            Table.rows[8].cells[2].innerHTML = list[File][Function].
            Table.rows[9].cells[2].innerHTML = list[File][Function].
            Table.rows[10].cells[2].innerHTML = list[File][Function].
            Table.rows[11].cells[2].innerHTML = list[File][Function].
            Table.rows[12].cells[2].innerHTML = list[File][Function].
            Table.rows[13].cells[2].innerHTML = list[File][Function].
            Table.rows[14].cells[2].innerHTML = list[File][Function].
            Table.rows[15].cells[2].innerHTML = list[File][Function].
            Table.rows[16].cells[2].innerHTML = list[File][Function].
            Table.rows[17].cells[2].innerHTML = list[File][Function].
            Table.rows[18].cells[2].innerHTML = list[File][Function].
            Table.rows[19].cells[2].innerHTML = list[File][Function].
            Table.rows[20].cells[2].innerHTML = list[File][Function].
            Table.rows[21].cells[3].innerHTML = list[File][Function].
            Table.rows[22].cells[2].innerHTML = list[File][Function].
            Table.rows[23].cells[2].innerHTML = list[File][Function].
            Table.rows[24].cells[2].innerHTML = list[File][Function].
            Table.rows[25].cells[2].innerHTML = list[File][Function].
            Table.rows[26].cells[3].innerHTML = list[File][Function].
            Table.rows[27].cells[2].innerHTML = list[File][Function].
            Table.rows[28].cells[2].innerHTML = list[File][Function].
            Table.rows[29].cells[3].innerHTML = list[File][Function].
            Table.rows[30].cells[2].innerHTML = list[File][Function].
            Table.rows[31].cells[2].innerHTML = list[File][Function].
            Table.rows[32].cells[3].innerHTML = list[File][Function].
            Table.rows[33].cells[2].innerHTML = list[File][Function].
            Table.rows[34].cells[2].innerHTML = list[File][Function].
            Table.rows[35].cells[2].innerHTML = list[File][Function].
        }}
        </script>
        ",
        list_a_items.to_dropbox_option_string(),
        list_b_items.to_dropbox_option_string(),
        sss,
        file_list,
    );
    template
}

fn read_yaml(space: String) -> Result<Vec<Doc>>{
    let mut docs = vec![];
    for d in space.split("---"){
        if d.is_empty(){continue;}
            let doc:Doc = from_str(&d)?;
            docs,push(doc);
    }
    Ok(docs)
}