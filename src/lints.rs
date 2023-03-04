use anyhow::Result;
use serde::Deserialize;
use std::fmt::{self, Display};

#[derive(Deserialize)]
pub(crate) struct Lint {
    pub id: String,
    pub group: LintGroup,
    pub level: LintLevel,
}

#[derive(Deserialize, PartialEq, Copy, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub(crate) enum LintGroup {
    Cargo,
    Complexity,
    Correctness,
    Deprecated,
    Nursery,
    Pedantic,
    Perf,
    Restriction,
    Style,
    Suspicious,
    Unknown,
}

#[derive(Deserialize, PartialEq, Copy, Clone)]
#[serde(rename_all = "lowercase")]
pub(crate) enum LintLevel {
    Allow,
    Warn,
    Deny,
    None,
}

impl Display for LintGroup {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str(&format!("{:?}", self).to_lowercase())
    }
}

pub(crate) fn download_lint_list() -> Result<Vec<Lint>> {
    let req = reqwest::blocking::get("https://rust-lang.github.io/rust-clippy/master/lints.json")?;
    let lints: Vec<Lint> = req.json()?;
    Ok(lints)
}

pub(crate) fn former_lint_group(lint_id: &str) -> Option<LintGroup> {
    match lint_id {
        // @dtolnay
        "cognitive_complexity" => Some(LintGroup::Complexity), // https://github.com/rust-lang/rust-clippy/pull/5428
        "implicit_hasher" => Some(LintGroup::Style), // https://github.com/rust-lang/rust-clippy/pull/5411
        "inefficient_to_string" => Some(LintGroup::Perf), // https://github.com/rust-lang/rust-clippy/pull/5412
        "integer_division" => Some(LintGroup::Pedantic), // https://github.com/rust-lang/rust-clippy/pull/4210
        "large_digit_groups" => Some(LintGroup::Style), // https://github.com/rust-lang/rust-clippy/pull/3479
        "let_unit_value" => Some(LintGroup::Style), // https://github.com/rust-lang/rust-clippy/pull/5409
        "manual_map" => Some(LintGroup::Style), // https://github.com/rust-lang/rust-clippy/pull/6796
        "many_single_char_names" => Some(LintGroup::Style), // https://github.com/rust-lang/rust-clippy/pull/7671
        "match_bool" => Some(LintGroup::Style), // https://github.com/rust-lang/rust-clippy/pull/5408
        "needless_pass_by_value" => Some(LintGroup::Style), // https://github.com/rust-lang/rust-clippy/pull/3439
        "new_ret_no_self" => Some(LintGroup::Style), // https://github.com/rust-lang/rust-clippy/pull/5420
        "nonstandard_macro_braces" => Some(LintGroup::Style), // https://github.com/rust-lang/rust-clippy/pull/7424
        "option_if_let_else" => Some(LintGroup::Pedantic), // https://github.com/rust-lang/rust-clippy/pull/7568
        "option_option" => Some(LintGroup::Complexity), // https://github.com/rust-lang/rust-clippy/pull/5401
        "rc_buffer" => Some(LintGroup::Perf), // https://github.com/rust-lang/rust-clippy/pull/6128
        "string_lit_as_bytes" => Some(LintGroup::Style), // https://github.com/rust-lang/rust-clippy/pull/6117
        "transmute_undefined_repr" => Some(LintGroup::Correctness), // https://github.com/rust-lang/rust-clippy/pull/8418
        "trivial_regex" => Some(LintGroup::Style), // https://github.com/rust-lang/rust-clippy/pull/6696
        "trivially_copy_pass_by_ref" => Some(LintGroup::Perf), // https://github.com/rust-lang/rust-clippy/pull/5410
        "unnested_or_patterns" => Some(LintGroup::Complexity), // https://github.com/rust-lang/rust-clippy/pull/5705
        "unreadable_literal" => Some(LintGroup::Style), // https://github.com/rust-lang/rust-clippy/pull/5419
        "unsafe_vector_initialization" => Some(LintGroup::Correctness), // https://github.com/rust-lang/rust-clippy/pull/3478
        "useless_let_if_seq" => Some(LintGroup::Style), // https://github.com/rust-lang/rust-clippy/pull/5599

        // others
        "assertions_on_result_states" => Some(LintGroup::Style), // https://github.com/rust-lang/rust-clippy/pull/9273
        "await_holding_lock" => Some(LintGroup::Correctness), // https://github.com/rust-lang/rust-clippy/pull/6354
        "await_holding_refcell_ref" => Some(LintGroup::Correctness), // https://github.com/rust-lang/rust-clippy/pull/6354
        "borrow_interior_mutable_const" => Some(LintGroup::Correctness), // https://github.com/rust-lang/rust-clippy/pull/6098
        "branches_sharing_code" => Some(LintGroup::Complexity), // https://github.com/rust-lang/rust-clippy/pull/7595
        "cast_ptr_alignment" => Some(LintGroup::Correctness), // https://github.com/rust-lang/rust-clippy/pull/5667
        "declare_interior_mutable_const" => Some(LintGroup::Correctness), // https://github.com/rust-lang/rust-clippy/pull/6098
        "derive_partial_eq_without_eq" => Some(LintGroup::Style), // https://github.com/rust-lang/rust-clippy/pull/9535
        "eval_order_dependence" => Some(LintGroup::Suspicious), // https://github.com/rust-lang/rust-clippy/pull/8621
        "float_cmp" => Some(LintGroup::Correctness), // https://github.com/rust-lang/rust-clippy/pull/7692
        "format_push_string" => Some(LintGroup::Perf), // https://github.com/rust-lang/rust-clippy/pull/9161
        "index_refutable_slice" => Some(LintGroup::Nursery), // https://github.com/rust-lang/rust-clippy/pull/9975
        "iter_with_drain" => Some(LintGroup::Perf), // https://github.com/rust-lang/rust-clippy/pull/8541
        "manual_clamp" => Some(LintGroup::Complexity), // https://github.com/rust-lang/rust-clippy/pull/10101
        "match_wild_err_arm" => Some(LintGroup::Style), // https://github.com/rust-lang/rust-clippy/pull/5622
        "mixed_read_write_in_expression" => Some(LintGroup::Suspicious), // https://github.com/rust-lang/rust-clippy/pull/8621
        "mutex_atomic" => Some(LintGroup::Perf), // https://github.com/rust-lang/rust-clippy/pull/8260
        "needless_collect" => Some(LintGroup::Perf), // https://github.com/rust-lang/rust-clippy/pull/9705
        "non_ascii_literal" => Some(LintGroup::Pedantic), // https://github.com/rust-lang/rust-clippy/pull/7907
        "range_plus_one" => Some(LintGroup::Complexity), // https://github.com/rust-lang/rust-clippy/pull/5057
        "significant_drop_in_scrutinee" => Some(LintGroup::Suspicious), // https://github.com/rust-lang/rust-clippy/pull/9302
        "suspicious_operation_groupings" => Some(LintGroup::Style), // https://github.com/rust-lang/rust-clippy/pull/7266
        "try_err" => Some(LintGroup::Style), // https://github.com/rust-lang/rust-clippy/pull/8544
        "unnecessary_safety_doc" => Some(LintGroup::Style), // https://github.com/rust-lang/rust-clippy/pull/9989
        "unnecessary_wraps" => Some(LintGroup::Complexity), // https://github.com/rust-lang/rust-clippy/pull/6765
        "verbose_bit_mask" => Some(LintGroup::Style), // https://github.com/rust-lang/rust-clippy/pull/6036

        _ => None,
    }
}
