use std::io;

#[derive(Clone, PartialEq, Eq)]
enum Tipo{
    Nat,
    Bool,
    Abs{
        par: Box<Tipo>,
        ret: Box<Tipo>
    },
    Nope,
    MalTipado
}

fn main() {
    let mut main_expr = String::new();
    io::stdin().read_line(&mut main_expr).unwrap();
    let parts: Vec<&str> = (main_expr.trim()).split(' ').collect();
    let (mut t, ni) = avaliar_expr(parts.clone(), 0, &mut None);
    if ni < parts.len(){t = Tipo::Nope}
    printa_tipo(t, false);
    print!("\n");
}

fn printa_tipo(t: Tipo, isparam: bool) -> (){
    match t{
        Tipo::Nat => print!("Nat"),    
        Tipo::Bool => print!("Bool"),
        Tipo::Abs{par, ret} => {print!("( ");printa_tipo(*par, true);printa_tipo(*ret, false);print!(" )")},
        Tipo::Nope => {print!("!"); return;},
        Tipo::MalTipado => {print!("-"); return;}
    }
    if isparam {
        print!(" -> ");
    }
}

fn check_digit(digit: &str) -> bool{
    digit.parse::<usize>().is_ok()
}

fn check_var(var: &str) -> bool{
    match var{
        "suc" => false,
        "pred" => false,
        "true" => false,
        "if" => false,
        "else" => false,
        "then" => false,
        "endif" => false,
        "ehzero" => false,
        "lambda" => false,
        "Nat" => false,
        "Bool" => false,
        _ => true
    }
}

fn check_type(expr: Vec<&str>, index: usize) -> (Tipo, usize){
    let mut ni = index;
    match expr.clone()[ni]{
        "Nat" => (Tipo::Nat, ni+1),
        "Bool" => (Tipo::Bool, ni+1),
        "(" => { ni = ni + 1;
            if ni >= expr.len(){return (Tipo::Nope, ni);}
            let tipopar: Tipo;
            (tipopar, ni) = check_type(expr.clone(), ni);
            if ni >= expr.len(){return (Tipo::Nope, ni);}
            match expr.clone()[ni]{
                "->" => ni = ni + 1,
                _ => return (Tipo::Nope, ni + 1)
            }
            if ni >= expr.len(){return (Tipo::Nope, ni);}
            let tiporet: Tipo;
            (tiporet, ni) = check_type(expr.clone(), ni);
            match expr.clone()[ni]{
                ")" => ni = ni + 1,
                _ => return (Tipo::Nope, ni + 1)
            }
            match (tipopar.clone(), tiporet.clone()){
                (Tipo::Nope, _) => return (Tipo::Nope, ni),
                (_, Tipo::Nope) => return (Tipo::Nope, ni),
                _ => return (Tipo::Abs { par: (Box::new(tipopar.clone())), ret: (Box::new(tiporet.clone())) }, ni)
            }
        }
        _ => (Tipo::Nope, ni)
    }
}

fn if_else<'a, 'b>(expr: Vec<&'a str>, index: usize, var_list: &'b mut Option<Vec<(&'b str, Tipo)>>) -> (Tipo, usize) where 'a: 'b{
    if index >= expr.len(){return (Tipo::Nope, index);}
    let (condtype, mut ni) = avaliar_expr(expr.clone(), index, &mut (var_list.clone()));
    if ni >= expr.len(){return (Tipo::Nope, ni);}
    match expr.clone()[ni]{
        "then" => ni = ni+1,
        _ => return  (Tipo::Nope, ni+1)
    }
    let maintype: Tipo;
    if ni >= expr.len(){return (Tipo::Nope, ni);}
    (maintype, ni) = avaliar_expr(expr.clone(), ni, &mut (var_list.clone()));
    if ni >= expr.len(){return (Tipo::Nope, ni);}
    match expr[ni]{
        "else" => ni = ni+1,
        _ => return  (Tipo::Nope, ni+1)
    }
    let othertype: Tipo;
    if ni >= expr.len(){return (Tipo::Nope, ni);}
    (othertype, ni) = avaliar_expr(expr.clone(), ni, &mut (var_list.clone()));
    if ni >= expr.len(){return (Tipo::Nope, ni);}
    match expr[ni]{
        "endif" => ni = ni+1,
        _ => return  (Tipo::Nope, ni+1)
    }
    match (condtype.clone(), maintype.clone(), othertype.clone()){
        (Tipo::Nope, _, _) => return (Tipo::Nope, ni),
        (_, Tipo::Nope, _) => return (Tipo::Nope, ni),
        (_, _, Tipo::Nope) => return (Tipo::Nope, ni),
        _ => if maintype.clone() != othertype.clone() {return (Tipo::MalTipado, ni);}
    }
    if condtype.clone() != Tipo::Bool{
        return (Tipo::MalTipado, ni);
    }
    return (maintype, ni);
}

fn app<'a, 'b>(expr: Vec<&'a str>, index: usize, var_list: &'b mut Option<Vec<(&'b str, Tipo)>>) -> (Tipo, usize) where 'a: 'b{
    if index >= expr.len(){return (Tipo::Nope, index);}
    let (functype, mut ni) = avaliar_expr(expr.clone(), index, &mut (var_list.clone()));
    if ni >= expr.len(){return (Tipo::Nope, ni);}
    let paramtype: Tipo;
    (paramtype, ni) = avaliar_expr(expr.clone(), ni, &mut (var_list.clone()));
    if ni >= expr.len(){return (Tipo::Nope, ni);}
    match expr.clone()[ni]{
        ")" => ni = ni+1,
        _ => return  (Tipo::Nope, ni+1)
    }
    match (functype.clone(), paramtype.clone()){
        (Tipo::Nope, _) => return (Tipo::Nope, ni),
        (_, Tipo::Nope) => return (Tipo::Nope, ni),
        _ => ()
    }
    match functype.clone(){
        Tipo::Abs{par, ret} => if *par == paramtype.clone() {return (*ret, ni)} else {return (Tipo::MalTipado, ni)},
        _ => return (Tipo::MalTipado, ni)
    }
}

fn abstraction<'a, 'b>(expr: Vec<&'a str>, index: usize, var_list: &'b mut Option<Vec<(&'b str, Tipo)>>) -> (Tipo, usize) where 'a: 'b{
    let mut ni: usize = index;
    let var: &str;
    if ni >= expr.len(){return (Tipo::Nope, ni);}
    if check_var(expr.clone()[ni]) {var = expr.clone()[ni]} else{return (Tipo::Nope, ni)};
    ni = ni + 1;
    if ni >= expr.len(){return (Tipo::Nope, ni);}
    match expr.clone()[ni]{
        ":" => ni = ni + 1,
        _ => return (Tipo::Nope, ni + 1)
    }
    let tipopar: Tipo;
    if ni >= expr.len(){return (Tipo::Nope, ni);}
    (tipopar, ni) = check_type(expr.clone(), ni);
    match tipopar.clone(){
        Tipo::MalTipado => return (Tipo::MalTipado, ni),
        Tipo::Nope => return (Tipo::Nope, ni),
        _ => ()
    }
    if ni >= expr.len(){return (Tipo::Nope, ni);}
    match expr.clone()[ni]{
        "." => ni = ni + 1,
        _ => return (Tipo::Nope, ni + 1)
    }
    match var_list{
        None => {
            let mut vecv: Vec<(&str, Tipo)> = Vec::new();
            vecv.push((var, tipopar.clone()));
            var_list.get_or_insert(vecv);
        }
        Some(varl) => varl.push((var, tipopar.clone()))
    }
    let tiporet: Tipo;
    if ni >= expr.len(){return (Tipo::Nope, ni);}
    (tiporet, ni) = avaliar_expr(expr.clone(), ni, &mut var_list.clone());
    match var_list{
        None => return (Tipo::Nope, ni),
        Some(varl) => varl.pop()
    };
    if ni >= expr.len(){return (Tipo::Nope, ni);}
    match expr[ni]{
        "end" => ni = ni+1,
        _ => return  (Tipo::Nope, ni+1)
    }
    match tiporet{
        Tipo::MalTipado => return (Tipo::MalTipado, ni),
        Tipo::Nope => return (Tipo::Nope, ni),
        _ => ()
    }
    (Tipo::Abs { par: Box::new(tipopar.clone()), ret: Box::new(tiporet.clone()) }, ni)
}

fn avaliar_expr<'a, 'b>(expr: Vec<&'a str>, index: usize, var_list: &'a mut Option<Vec<(&'b str, Tipo)>>) -> (Tipo, usize) where 'a: 'b{
    let mut t: Tipo = Tipo::Nope;
    let mut ni = index;
    if ni >= expr.len(){return (Tipo::Nope, ni);}
    match expr.clone()[ni]{
    "true" => {t = Tipo::Bool; ni = ni + 1},
    "false" => {t = Tipo::Bool; ni = ni + 1},
    "if" => (t, ni) = if_else(expr.clone(), index+1, var_list),
    "suc" => {t = Tipo::Abs{par: Box::new(Tipo::Nat), ret: Box::new(Tipo::Nat)}; ni = ni + 1},
    "pred" => {t = Tipo::Abs{par: Box::new(Tipo::Nat), ret: Box::new(Tipo::Nat)}; ni = ni + 1},
    "ehzero" => {t = Tipo::Abs{par: Box::new(Tipo::Nat), ret: Box::new(Tipo::Bool)}; ni = ni + 1}, 
    "lambda" => (t, ni) = abstraction(expr.clone(), index+1, var_list),
    "(" => (t, ni) = app(expr.clone(), index+1, var_list),
    icognita => {
        match var_list{
            Some(v) => 
            if check_digit(icognita){
                return (Tipo::Nat, ni + 1);
            } else{
                for i in v.iter().rev(){
                    let (nom, tip) = i;
                    if nom == &icognita{
                        t = tip.clone();
                        return (t, ni + 1);
                    }
                }
                if contexto_vazio(expr.clone()[ni]){
                    return (Tipo::MalTipado, ni + 1);
                }
            },
            None => if check_digit(icognita){
                return (Tipo::Nat, ni + 1);
            } else{
                if contexto_vazio(expr.clone()[ni]){
                    return (Tipo::MalTipado, ni + 1);
                }
            }
        }
    }
    }
    (t, ni)
}

fn contexto_vazio(p_var: &str) -> bool{
    return p_var.chars().all(|c| "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789".contains(c));
}