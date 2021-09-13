## How often is each Clippy lint suppressed on crates.io

Clippy's most severe flaw in my experience has been low-signal lints that are
enabled by default, aren't worth resolving and commonly need to be suppressed.

I use Clippy across a large number of my Rust projects, so I already get good
visibility into which lints are misguided, buggy, or have unacceptably low true
positive rate on real world code. One of my [hobbies] apparently is deleting
such lints from Clippy or downgrading them out of the set of enabled-by-default
lints (opt-out lints) to the `pedantic` or `restriction` (opt-in) groups
instead.

This repo contains a script for analyzing suppressed lints on a bigger corpus:
all of crates.io. For every Clippy lint, the program counts how many times it is
suppressed globally (at module scope) or locally (on one single place the lint
is triggered).

In the table below, I would recommend paying attention to the **style** and
**perf** lints. Highly suppressed **style** lints indicate that the community
has consciously decided that Clippy's opinion on style is wrong. Highly
suppressed **perf** lints indicate that the community does not consider it
valuable to make their code more obtuse for the sake of questionable alleged
performance. I think it would be wise to delete or downgrade many of these.

[hobbies]: https://github.com/rust-lang/rust-clippy/pulls?q=is%3Apr+is%3Amerged+author%3Adtolnay+downgrade

<br>

## Results (updated September 2021)

global | local | lint name | category
--- | --- | --- | ---
423 | 3283 | **[type_complexity](https://rust-lang.github.io/rust-clippy/master/index.html#type_complexity)** | complexity
430 | 2326 | **[too_many_arguments](https://rust-lang.github.io/rust-clippy/master/index.html#too_many_arguments)** | complexity
78 | 2167 | **[wrong_self_convention](https://rust-lang.github.io/rust-clippy/master/index.html#wrong_self_convention)** | style
390 | 827 | **[upper_case_acronyms](https://rust-lang.github.io/rust-clippy/master/index.html#upper_case_acronyms)** | style
966 | 236 | **[all](https://rust-lang.github.io/rust-clippy/master/index.html#all)** | unknown
~*68*~ | ~*837*~ | ~*[cast_ptr_alignment](https://rust-lang.github.io/rust-clippy/master/index.html#cast_ptr_alignment)*~ | ~*correctness*~ pedantic
274 | 616 | **[many_single_char_names](https://rust-lang.github.io/rust-clippy/master/index.html#many_single_char_names)** | style
~*131*~ | ~*707*~ | ~*[cognitive_complexity](https://rust-lang.github.io/rust-clippy/master/index.html#cognitive_complexity)*~ | ~*complexity*~ nursery
120 | 700 | **[large_enum_variant](https://rust-lang.github.io/rust-clippy/master/index.html#large_enum_variant)** | perf
~*768*~ | ~*27*~ | ~*[let_unit_value](https://rust-lang.github.io/rust-clippy/master/index.html#let_unit_value)*~ | ~*style*~ pedantic
~*154*~ | ~*602*~ | ~*[cast_possible_truncation](https://rust-lang.github.io/rust-clippy/master/index.html#cast_possible_truncation)*~ | pedantic
~*405*~ | ~*325*~ | ~*[unreadable_literal](https://rust-lang.github.io/rust-clippy/master/index.html#unreadable_literal)*~ | ~*style*~ pedantic
159 | 474 | **[float_cmp](https://rust-lang.github.io/rust-clippy/master/index.html#float_cmp)** | correctness
~*376*~ | ~*247*~ | ~*[module_name_repetitions](https://rust-lang.github.io/rust-clippy/master/index.html#module_name_repetitions)*~ | pedantic
~*86*~ | ~*495*~ | ~*[trivially_copy_pass_by_ref](https://rust-lang.github.io/rust-clippy/master/index.html#trivially_copy_pass_by_ref)*~ | ~*perf*~ pedantic
78 | 418 | **[identity_op](https://rust-lang.github.io/rust-clippy/master/index.html#identity_op)** | complexity
~*29*~ | ~*458*~ | ~*[unwrap_used](https://rust-lang.github.io/rust-clippy/master/index.html#unwrap_used)*~ | restriction
73 | 396 | **[new_ret_no_self](https://rust-lang.github.io/rust-clippy/master/index.html#new_ret_no_self)** | style
~*99*~ | ~*358*~ | ~*[cast_sign_loss](https://rust-lang.github.io/rust-clippy/master/index.html#cast_sign_loss)*~ | pedantic
~*64*~ | ~*393*~ | ~*[unnecessary_wraps](https://rust-lang.github.io/rust-clippy/master/index.html#unnecessary_wraps)*~ | ~*complexity*~ pedantic
225 | 229 | **[missing_safety_doc](https://rust-lang.github.io/rust-clippy/master/index.html#missing_safety_doc)** | style
102 | 331 | **[should_implement_trait](https://rust-lang.github.io/rust-clippy/master/index.html#should_implement_trait)** | style
~*97*~ | ~*320*~ | ~*[needless_pass_by_value](https://rust-lang.github.io/rust-clippy/master/index.html#needless_pass_by_value)*~ | ~*style*~ pedantic
168 | 246 | **[new_without_default](https://rust-lang.github.io/rust-clippy/master/index.html#new_without_default)** | style
~*34*~ | ~*338*~ | ~*[missing_inline_in_public_items](https://rust-lang.github.io/rust-clippy/master/index.html#missing_inline_in_public_items)*~ | restriction
~*103*~ | ~*265*~ | ~*[too_many_lines](https://rust-lang.github.io/rust-clippy/master/index.html#too_many_lines)*~ | pedantic
57 | 279 | **[len_without_is_empty](https://rust-lang.github.io/rust-clippy/master/index.html#len_without_is_empty)** | style
111 | 202 | **[module_inception](https://rust-lang.github.io/rust-clippy/master/index.html#module_inception)** | style
~*142*~ | ~*157*~ | ~*[integer_arithmetic](https://rust-lang.github.io/rust-clippy/master/index.html#integer_arithmetic)*~ | restriction
56 | 221 | **[needless_range_loop](https://rust-lang.github.io/rust-clippy/master/index.html#needless_range_loop)** | style
52 | 225 | **[ptr_arg](https://rust-lang.github.io/rust-clippy/master/index.html#ptr_arg)** | style
~*85*~ | ~*191*~ | ~*[use_self](https://rust-lang.github.io/rust-clippy/master/index.html#use_self)*~ | nursery
227 | 33 | **[needless_doctest_main](https://rust-lang.github.io/rust-clippy/master/index.html#needless_doctest_main)** | style
43 | 217 | **[suspicious_arithmetic_impl](https://rust-lang.github.io/rust-clippy/master/index.html#suspicious_arithmetic_impl)** | suspicious
~*83*~ | ~*160*~ | ~*[cast_possible_wrap](https://rust-lang.github.io/rust-clippy/master/index.html#cast_possible_wrap)*~ | pedantic
139 | 104 | **[from_over_into](https://rust-lang.github.io/rust-clippy/master/index.html#from_over_into)** | style
109 | 132 | **[excessive_precision](https://rust-lang.github.io/rust-clippy/master/index.html#excessive_precision)** | style
~*205*~ | ~*34*~ | ~*[missing_errors_doc](https://rust-lang.github.io/rust-clippy/master/index.html#missing_errors_doc)*~ | pedantic
~*47*~ | ~*187*~ | ~*[missing_const_for_fn](https://rust-lang.github.io/rust-clippy/master/index.html#missing_const_for_fn)*~ | nursery
~*197*~ | ~*37*~ | ~*[must_use_candidate](https://rust-lang.github.io/rust-clippy/master/index.html#must_use_candidate)*~ | pedantic
~*67*~ | ~*153*~ | ~*[cast_precision_loss](https://rust-lang.github.io/rust-clippy/master/index.html#cast_precision_loss)*~ | pedantic
71 | 139 | **[redundant_closure](https://rust-lang.github.io/rust-clippy/master/index.html#redundant_closure)** | style
91 | 111 | **[not_unsafe_ptr_arg_deref](https://rust-lang.github.io/rust-clippy/master/index.html#not_unsafe_ptr_arg_deref)** | correctness
58 | 140 | **[single_match](https://rust-lang.github.io/rust-clippy/master/index.html#single_match)** | style
~*105*~ | ~*90*~ | ~*[cast_lossless](https://rust-lang.github.io/rust-clippy/master/index.html#cast_lossless)*~ | pedantic
70 | 123 | **[let_and_return](https://rust-lang.github.io/rust-clippy/master/index.html#let_and_return)** | style
54 | 137 | **[needless_lifetimes](https://rust-lang.github.io/rust-clippy/master/index.html#needless_lifetimes)** | complexity
16 | 170 | **[useless_conversion](https://rust-lang.github.io/rust-clippy/master/index.html#useless_conversion)** | complexity
91 | 93 | **[match_single_binding](https://rust-lang.github.io/rust-clippy/master/index.html#match_single_binding)** | complexity
~*23*~ | ~*159*~ | ~*[unused_self](https://rust-lang.github.io/rust-clippy/master/index.html#unused_self)*~ | pedantic
~*117*~ | ~*65*~ | ~*[wildcard_imports](https://rust-lang.github.io/rust-clippy/master/index.html#wildcard_imports)*~ | pedantic
~*111*~ | ~*69*~ | ~*[similar_names](https://rust-lang.github.io/rust-clippy/master/index.html#similar_names)*~ | pedantic
3 | 172 | **[mut_from_ref](https://rust-lang.github.io/rust-clippy/master/index.html#mut_from_ref)** | correctness
38 | 137 | **[unknown_clippy_lints](https://rust-lang.github.io/rust-clippy/master/index.html#unknown_clippy_lints)** | unknown
~*52*~ | ~*116*~ | ~*[missing_panics_doc](https://rust-lang.github.io/rust-clippy/master/index.html#missing_panics_doc)*~ | pedantic
37 | 127 | **[eq_op](https://rust-lang.github.io/rust-clippy/master/index.html#eq_op)** | correctness
~*126*~ | ~*29*~ | ~*[doc_markdown](https://rust-lang.github.io/rust-clippy/master/index.html#doc_markdown)*~ | pedantic
8 | 144 | **[clone_on_copy](https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy)** | complexity
89 | 62 | **[approx_constant](https://rust-lang.github.io/rust-clippy/master/index.html#approx_constant)** | correctness
42 | 107 | **[collapsible_if](https://rust-lang.github.io/rust-clippy/master/index.html#collapsible_if)** | style
29 | 117 | **[redundant_clone](https://rust-lang.github.io/rust-clippy/master/index.html#redundant_clone)** | perf
~*29*~ | ~*116*~ | ~*[redundant_pub_crate](https://rust-lang.github.io/rust-clippy/master/index.html#redundant_pub_crate)*~ | nursery
114 | 26 | **[needless_return](https://rust-lang.github.io/rust-clippy/master/index.html#needless_return)** | style
~*43*~ | ~*95*~ | ~*[match_same_arms](https://rust-lang.github.io/rust-clippy/master/index.html#match_same_arms)*~ | pedantic
~*33*~ | ~*105*~ | ~*[type_repetition_in_bounds](https://rust-lang.github.io/rust-clippy/master/index.html#type_repetition_in_bounds)*~ | pedantic
22 | 115 | **[enum_variant_names](https://rust-lang.github.io/rust-clippy/master/index.html#enum_variant_names)** | style
~*90*~ | ~*42*~ | ~*[non_ascii_literal](https://rust-lang.github.io/rust-clippy/master/index.html#non_ascii_literal)*~ | pedantic
20 | 111 | **[derive_hash_xor_eq](https://rust-lang.github.io/rust-clippy/master/index.html#derive_hash_xor_eq)** | correctness
~*58*~ | ~*73*~ | ~*[option_if_let_else](https://rust-lang.github.io/rust-clippy/master/index.html#option_if_let_else)*~ | ~*pedantic*~ nursery
31 | 90 | **[comparison_chain](https://rust-lang.github.io/rust-clippy/master/index.html#comparison_chain)** | style
61 | 59 | **[match_like_matches_macro](https://rust-lang.github.io/rust-clippy/master/index.html#match_like_matches_macro)** | style
~*33*~ | ~*84*~ | ~*[used_underscore_binding](https://rust-lang.github.io/rust-clippy/master/index.html#used_underscore_binding)*~ | pedantic
~*36*~ | ~*79*~ | ~*[transmute_ptr_to_ptr](https://rust-lang.github.io/rust-clippy/master/index.html#transmute_ptr_to_ptr)*~ | pedantic
94 | 21 | **[unused_unit](https://rust-lang.github.io/rust-clippy/master/index.html#unused_unit)** | style
~*57*~ | ~*57*~ | ~*[shadow_unrelated](https://rust-lang.github.io/rust-clippy/master/index.html#shadow_unrelated)*~ | pedantic
~*110*~ | ~*0*~ | ~*[multiple_crate_versions](https://rust-lang.github.io/rust-clippy/master/index.html#multiple_crate_versions)*~ | cargo
~*4*~ | ~*105*~ | ~*[expect_used](https://rust-lang.github.io/rust-clippy/master/index.html#expect_used)*~ | restriction
~*49*~ | ~*57*~ | ~*[missing_docs_in_private_items](https://rust-lang.github.io/rust-clippy/master/index.html#missing_docs_in_private_items)*~ | restriction
19 | 85 | **[if_same_then_else](https://rust-lang.github.io/rust-clippy/master/index.html#if_same_then_else)** | correctness
37 | 67 | **[mutex_atomic](https://rust-lang.github.io/rust-clippy/master/index.html#mutex_atomic)** | perf
~*47*~ | ~*55*~ | ~*[indexing_slicing](https://rust-lang.github.io/rust-clippy/master/index.html#indexing_slicing)*~ | restriction
56 | 46 | **[unit_arg](https://rust-lang.github.io/rust-clippy/master/index.html#unit_arg)** | complexity
12 | 84 | **[borrowed_box](https://rust-lang.github.io/rust-clippy/master/index.html#borrowed_box)** | complexity
69 | 25 | **[field_reassign_with_default](https://rust-lang.github.io/rust-clippy/master/index.html#field_reassign_with_default)** | style
~*77*~ | ~*15*~ | ~*[default_trait_access](https://rust-lang.github.io/rust-clippy/master/index.html#default_trait_access)*~ | pedantic
16 | 76 | **[result_unit_err](https://rust-lang.github.io/rust-clippy/master/index.html#result_unit_err)** | style
5 | 86 | **[uninit_assumed_init](https://rust-lang.github.io/rust-clippy/master/index.html#uninit_assumed_init)** | correctness
~*10*~ | ~*79*~ | ~*[mut_mut](https://rust-lang.github.io/rust-clippy/master/index.html#mut_mut)*~ | pedantic
~*48*~ | ~*39*~ | ~*[inline_always](https://rust-lang.github.io/rust-clippy/master/index.html#inline_always)*~ | pedantic
~*17*~ | ~*69*~ | ~*[exhaustive_structs](https://rust-lang.github.io/rust-clippy/master/index.html#exhaustive_structs)*~ | restriction
34 | 51 | **[suspicious_op_assign_impl](https://rust-lang.github.io/rust-clippy/master/index.html#suspicious_op_assign_impl)** | suspicious
5 | 79 | **[erasing_op](https://rust-lang.github.io/rust-clippy/master/index.html#erasing_op)** | correctness
~*29*~ | ~*54*~ | ~*[as_conversions](https://rust-lang.github.io/rust-clippy/master/index.html#as_conversions)*~ | restriction
33 | 50 | **[never_loop](https://rust-lang.github.io/rust-clippy/master/index.html#never_loop)** | correctness
~*25*~ | ~*57*~ | ~*[implicit_hasher](https://rust-lang.github.io/rust-clippy/master/index.html#implicit_hasher)*~ | ~*style*~ pedantic
7 | 74 | **[map_entry](https://rust-lang.github.io/rust-clippy/master/index.html#map_entry)** | perf
32 | 49 | **[or_fun_call](https://rust-lang.github.io/rust-clippy/master/index.html#or_fun_call)** | perf
17 | 61 | **[assertions_on_constants](https://rust-lang.github.io/rust-clippy/master/index.html#assertions_on_constants)** | style
11 | 66 | **[needless_collect](https://rust-lang.github.io/rust-clippy/master/index.html#needless_collect)** | perf
37 | 40 | **[op_ref](https://rust-lang.github.io/rust-clippy/master/index.html#op_ref)** | style
~*8*~ | ~*68*~ | ~*[pattern_type_mismatch](https://rust-lang.github.io/rust-clippy/master/index.html#pattern_type_mismatch)*~ | restriction
~*60*~ | ~*16*~ | ~*[print_stdout](https://rust-lang.github.io/rust-clippy/master/index.html#print_stdout)*~ | restriction
28 | 47 | **[suspicious_else_formatting](https://rust-lang.github.io/rust-clippy/master/index.html#suspicious_else_formatting)** | suspicious
~*20*~ | ~*55*~ | ~*[wildcard_enum_match_arm](https://rust-lang.github.io/rust-clippy/master/index.html#wildcard_enum_match_arm)*~ | restriction
~*15*~ | ~*56*~ | ~*[range_plus_one](https://rust-lang.github.io/rust-clippy/master/index.html#range_plus_one)*~ | ~*complexity*~ pedantic
13 | 58 | **[reversed_empty_ranges](https://rust-lang.github.io/rust-clippy/master/index.html#reversed_empty_ranges)** | correctness
51 | 18 | **[assign_op_pattern](https://rust-lang.github.io/rust-clippy/master/index.html#assign_op_pattern)** | style
~*21*~ | ~*48*~ | ~*[struct_excessive_bools](https://rust-lang.github.io/rust-clippy/master/index.html#struct_excessive_bools)*~ | pedantic
~*10*~ | ~*58*~ | ~*[rc_buffer](https://rust-lang.github.io/rust-clippy/master/index.html#rc_buffer)*~ | ~*perf*~ restriction
~*3*~ | ~*64*~ | ~*[suspicious_operation_groupings](https://rust-lang.github.io/rust-clippy/master/index.html#suspicious_operation_groupings)*~ | ~*style*~ nursery
~*47*~ | ~*19*~ | ~*[if_not_else](https://rust-lang.github.io/rust-clippy/master/index.html#if_not_else)*~ | pedantic
~*11*~ | ~*55*~ | ~*[option_option](https://rust-lang.github.io/rust-clippy/master/index.html#option_option)*~ | ~*complexity*~ pedantic
46 | 20 | **[redundant_field_names](https://rust-lang.github.io/rust-clippy/master/index.html#redundant_field_names)** | style
~*52*~ | ~*14*~ | ~*[single_match_else](https://rust-lang.github.io/rust-clippy/master/index.html#single_match_else)*~ | pedantic
~*44*~ | ~*21*~ | ~*[enum_glob_use](https://rust-lang.github.io/rust-clippy/master/index.html#enum_glob_use)*~ | pedantic
6 | 58 | **[empty_loop](https://rust-lang.github.io/rust-clippy/master/index.html#empty_loop)** | suspicious
18 | 46 | **[inconsistent_digit_grouping](https://rust-lang.github.io/rust-clippy/master/index.html#inconsistent_digit_grouping)** | style
~*42*~ | ~*22*~ | ~*[match_bool](https://rust-lang.github.io/rust-clippy/master/index.html#match_bool)*~ | ~*style*~ pedantic
10 | 54 | **[mutable_key_type](https://rust-lang.github.io/rust-clippy/master/index.html#mutable_key_type)** | suspicious
~*62*~ | ~*0*~ | ~*[implicit_return](https://rust-lang.github.io/rust-clippy/master/index.html#implicit_return)*~ | restriction
31 | 30 | **[blacklisted_name](https://rust-lang.github.io/rust-clippy/master/index.html#blacklisted_name)** | style
~*0*~ | ~*61*~ | ~*[ref_option_ref](https://rust-lang.github.io/rust-clippy/master/index.html#ref_option_ref)*~ | pedantic
6 | 53 | **[clone_double_ref](https://rust-lang.github.io/rust-clippy/master/index.html#clone_double_ref)** | correctness
36 | 21 | **[borrow_interior_mutable_const](https://rust-lang.github.io/rust-clippy/master/index.html#borrow_interior_mutable_const)** | ~*correctness*~ style
8 | 49 | **[declare_interior_mutable_const](https://rust-lang.github.io/rust-clippy/master/index.html#declare_interior_mutable_const)** | ~*correctness*~ style
~*7*~ | ~*50*~ | ~*[match_wild_err_arm](https://rust-lang.github.io/rust-clippy/master/index.html#match_wild_err_arm)*~ | ~*style*~ pedantic
48 | 9 | **[redundant_static_lifetimes](https://rust-lang.github.io/rust-clippy/master/index.html#redundant_static_lifetimes)** | style
13 | 44 | **[zero_prefixed_literal](https://rust-lang.github.io/rust-clippy/master/index.html#zero_prefixed_literal)** | complexity
~*12*~ | ~*44*~ | ~*[semicolon_if_nothing_returned](https://rust-lang.github.io/rust-clippy/master/index.html#semicolon_if_nothing_returned)*~ | pedantic
14 | 41 | **[eval_order_dependence](https://rust-lang.github.io/rust-clippy/master/index.html#eval_order_dependence)** | suspicious
18 | 36 | **[collapsible_else_if](https://rust-lang.github.io/rust-clippy/master/index.html#collapsible_else_if)** | style
0 | 53 | **[drop_bounds](https://rust-lang.github.io/rust-clippy/master/index.html#drop_bounds)** | unknown
21 | 32 | **[unnecessary_cast](https://rust-lang.github.io/rust-clippy/master/index.html#unnecessary_cast)** | complexity
13 | 39 | **[ptr_offset_with_cast](https://rust-lang.github.io/rust-clippy/master/index.html#ptr_offset_with_cast)** | complexity
29 | 23 | **[pub_enum_variant_names](https://rust-lang.github.io/rust-clippy/master/index.html#pub_enum_variant_names)** | deprecated
39 | 13 | **[redundant_closure_call](https://rust-lang.github.io/rust-clippy/master/index.html#redundant_closure_call)** | complexity
33 | 18 | **[manual_range_contains](https://rust-lang.github.io/rust-clippy/master/index.html#manual_range_contains)** | style
11 | 40 | **[useless_attribute](https://rust-lang.github.io/rust-clippy/master/index.html#useless_attribute)** | correctness
24 | 24 | **[deref_addrof](https://rust-lang.github.io/rust-clippy/master/index.html#deref_addrof)** | complexity
~*6*~ | ~*42*~ | ~*[let_underscore_must_use](https://rust-lang.github.io/rust-clippy/master/index.html#let_underscore_must_use)*~ | restriction
3 | 45 | **[same_item_push](https://rust-lang.github.io/rust-clippy/master/index.html#same_item_push)** | style
~*8*~ | ~*38*~ | ~*[exhaustive_enums](https://rust-lang.github.io/rust-clippy/master/index.html#exhaustive_enums)*~ | restriction
~*34*~ | ~*12*~ | ~*[items_after_statements](https://rust-lang.github.io/rust-clippy/master/index.html#items_after_statements)*~ | pedantic
~*6*~ | ~*39*~ | ~*[branches_sharing_code](https://rust-lang.github.io/rust-clippy/master/index.html#branches_sharing_code)*~ | nursery
7 | 38 | **[map_clone](https://rust-lang.github.io/rust-clippy/master/index.html#map_clone)** | style
7 | 37 | **[cyclomatic_complexity](https://rust-lang.github.io/rust-clippy/master/index.html#cyclomatic_complexity)** | unknown
43 | 1 | **[tabs_in_doc_comments](https://rust-lang.github.io/rust-clippy/master/index.html#tabs_in_doc_comments)** | style
~*24*~ | ~*19*~ | ~*[panic](https://rust-lang.github.io/rust-clippy/master/index.html#panic)*~ | restriction
7 | 35 | **[boxed_local](https://rust-lang.github.io/rust-clippy/master/index.html#boxed_local)** | perf
~*27*~ | ~*14*~ | ~*[float_arithmetic](https://rust-lang.github.io/rust-clippy/master/index.html#float_arithmetic)*~ | restriction
3 | 38 | **[while_let_on_iterator](https://rust-lang.github.io/rust-clippy/master/index.html#while_let_on_iterator)** | style
12 | 28 | **[blocks_in_if_conditions](https://rust-lang.github.io/rust-clippy/master/index.html#blocks_in_if_conditions)** | style
14 | 26 | **[manual_map](https://rust-lang.github.io/rust-clippy/master/index.html#manual_map)** | style
12 | 28 | **[manual_strip](https://rust-lang.github.io/rust-clippy/master/index.html#manual_strip)** | complexity
~*13*~ | ~*27*~ | ~*[map_err_ignore](https://rust-lang.github.io/rust-clippy/master/index.html#map_err_ignore)*~ | restriction
~*27*~ | ~*13*~ | ~*[useless_transmute](https://rust-lang.github.io/rust-clippy/master/index.html#useless_transmute)*~ | nursery
~*38*~ | ~*1*~ | ~*[nonstandard_macro_braces](https://rust-lang.github.io/rust-clippy/master/index.html#nonstandard_macro_braces)*~ | ~*style*~ nursery
30 | 9 | **[pedantic](https://rust-lang.github.io/rust-clippy/master/index.html#pedantic)** | unknown
6 | 33 | **[unnecessary_unwrap](https://rust-lang.github.io/rust-clippy/master/index.html#unnecessary_unwrap)** | complexity
5 | 34 | **[vec_box](https://rust-lang.github.io/rust-clippy/master/index.html#vec_box)** | complexity
20 | 18 | **[write_with_newline](https://rust-lang.github.io/rust-clippy/master/index.html#write_with_newline)** | style
~*0*~ | ~*38*~ | ~*[zero_sized_map_values](https://rust-lang.github.io/rust-clippy/master/index.html#zero_sized_map_values)*~ | pedantic
~*37*~ | ~*0*~ | ~*[dbg_macro](https://rust-lang.github.io/rust-clippy/master/index.html#dbg_macro)*~ | restriction
11 | 26 | **[redundant_pattern_matching](https://rust-lang.github.io/rust-clippy/master/index.html#redundant_pattern_matching)** | style
~*22*~ | ~*15*~ | ~*[shadow_reuse](https://rust-lang.github.io/rust-clippy/master/index.html#shadow_reuse)*~ | restriction
6 | 30 | **[absurd_extreme_comparisons](https://rust-lang.github.io/rust-clippy/master/index.html#absurd_extreme_comparisons)** | correctness
~*17*~ | ~*19*~ | ~*[useless_let_if_seq](https://rust-lang.github.io/rust-clippy/master/index.html#useless_let_if_seq)*~ | ~*style*~ nursery
1 | 34 | **[cast_ref_to_mut](https://rust-lang.github.io/rust-clippy/master/index.html#cast_ref_to_mut)** | correctness
17 | 18 | **[filter_map](https://rust-lang.github.io/rust-clippy/master/index.html#filter_map)** | deprecated
19 | 16 | **[single_component_path_imports](https://rust-lang.github.io/rust-clippy/master/index.html#single_component_path_imports)** | style
~*25*~ | ~*10*~ | ~*[string_lit_as_bytes](https://rust-lang.github.io/rust-clippy/master/index.html#string_lit_as_bytes)*~ | ~*style*~ nursery
~*9*~ | ~*26*~ | ~*[trivial_regex](https://rust-lang.github.io/rust-clippy/master/index.html#trivial_regex)*~ | ~*style*~ nursery
~*22*~ | ~*12*~ | ~*[integer_division](https://rust-lang.github.io/rust-clippy/master/index.html#integer_division)*~ | ~*pedantic*~ restriction
13 | 21 | **[len_zero](https://rust-lang.github.io/rust-clippy/master/index.html#len_zero)** | style
~*32*~ | ~*2*~ | ~*[unseparated_literal_suffix](https://rust-lang.github.io/rust-clippy/master/index.html#unseparated_literal_suffix)*~ | pedantic
20 | 14 | **[unusual_byte_groupings](https://rust-lang.github.io/rust-clippy/master/index.html#unusual_byte_groupings)** | style
8 | 25 | **[needless_bool](https://rust-lang.github.io/rust-clippy/master/index.html#needless_bool)** | complexity
~*22*~ | ~*11*~ | ~*[unneeded_field_pattern](https://rust-lang.github.io/rust-clippy/master/index.html#unneeded_field_pattern)*~ | restriction
2 | 30 | **[extra_unused_lifetimes](https://rust-lang.github.io/rust-clippy/master/index.html#extra_unused_lifetimes)** | complexity
31 | 0 | **[blanket_clippy_restriction_lints](https://rust-lang.github.io/rust-clippy/master/index.html#blanket_clippy_restriction_lints)** | suspicious
6 | 25 | **[partialeq_ne_impl](https://rust-lang.github.io/rust-clippy/master/index.html#partialeq_ne_impl)** | complexity
0 | 30 | **[derive_ord_xor_partial_ord](https://rust-lang.github.io/rust-clippy/master/index.html#derive_ord_xor_partial_ord)** | correctness
~*26*~ | ~*3*~ | ~*[else_if_without_else](https://rust-lang.github.io/rust-clippy/master/index.html#else_if_without_else)*~ | restriction
4 | 25 | **[forget_copy](https://rust-lang.github.io/rust-clippy/master/index.html#forget_copy)** | correctness
11 | 18 | **[vec_init_then_push](https://rust-lang.github.io/rust-clippy/master/index.html#vec_init_then_push)** | perf
27 | 1 | **[deprecated_cfg_attr](https://rust-lang.github.io/rust-clippy/master/index.html#deprecated_cfg_attr)** | complexity
~*25*~ | ~*3*~ | ~*[let_underscore_drop](https://rust-lang.github.io/rust-clippy/master/index.html#let_underscore_drop)*~ | pedantic
9 | 19 | **[nonminimal_bool](https://rust-lang.github.io/rust-clippy/master/index.html#nonminimal_bool)** | complexity
~*19*~ | ~*9*~ | ~*[unnested_or_patterns](https://rust-lang.github.io/rust-clippy/master/index.html#unnested_or_patterns)*~ | ~*complexity*~ pedantic
10 | 16 | **[manual_non_exhaustive](https://rust-lang.github.io/rust-clippy/master/index.html#manual_non_exhaustive)** | style
11 | 15 | **[needless_update](https://rust-lang.github.io/rust-clippy/master/index.html#needless_update)** | complexity
12 | 14 | **[suspicious_map](https://rust-lang.github.io/rust-clippy/master/index.html#suspicious_map)** | suspicious
13 | 12 | **[bool_comparison](https://rust-lang.github.io/rust-clippy/master/index.html#bool_comparison)** | complexity
~*15*~ | ~*10*~ | ~*[match_wildcard_for_single_variants](https://rust-lang.github.io/rust-clippy/master/index.html#match_wildcard_for_single_variants)*~ | pedantic
~*12*~ | ~*12*~ | ~*[fallible_impl_from](https://rust-lang.github.io/rust-clippy/master/index.html#fallible_impl_from)*~ | nursery
11 | 13 | **[print_literal](https://rust-lang.github.io/rust-clippy/master/index.html#print_literal)** | style
~*23*~ | ~*1*~ | ~*[redundant_else](https://rust-lang.github.io/rust-clippy/master/index.html#redundant_else)*~ | pedantic
~*10*~ | ~*13*~ | ~*[empty_enum](https://rust-lang.github.io/rust-clippy/master/index.html#empty_enum)*~ | pedantic
1 | 22 | **[into_iter_on_ref](https://rust-lang.github.io/rust-clippy/master/index.html#into_iter_on_ref)** | style
8 | 15 | **[neg_cmp_op_on_partial_ord](https://rust-lang.github.io/rust-clippy/master/index.html#neg_cmp_op_on_partial_ord)** | complexity
16 | 7 | **[toplevel_ref_arg](https://rust-lang.github.io/rust-clippy/master/index.html#toplevel_ref_arg)** | style
4 | 18 | **[drop_copy](https://rust-lang.github.io/rust-clippy/master/index.html#drop_copy)** | correctness
11 | 11 | **[expect_fun_call](https://rust-lang.github.io/rust-clippy/master/index.html#expect_fun_call)** | perf
10 | 12 | **[inherent_to_string](https://rust-lang.github.io/rust-clippy/master/index.html#inherent_to_string)** | style
7 | 14 | **[explicit_counter_loop](https://rust-lang.github.io/rust-clippy/master/index.html#explicit_counter_loop)** | complexity
1 | 20 | **[no_effect](https://rust-lang.github.io/rust-clippy/master/index.html#no_effect)** | complexity
~*8*~ | ~*13*~ | ~*[suboptimal_flops](https://rust-lang.github.io/rust-clippy/master/index.html#suboptimal_flops)*~ | nursery
~*7*~ | ~*14*~ | ~*[unimplemented](https://rust-lang.github.io/rust-clippy/master/index.html#unimplemented)*~ | restriction
11 | 10 | **[unit_cmp](https://rust-lang.github.io/rust-clippy/master/index.html#unit_cmp)** | correctness
11 | 10 | **[unnecessary_operation](https://rust-lang.github.io/rust-clippy/master/index.html#unnecessary_operation)** | complexity
~*20*~ | ~*1*~ | ~*[unreachable](https://rust-lang.github.io/rust-clippy/master/index.html#unreachable)*~ | restriction
4 | 17 | **[wildcard_in_or_patterns](https://rust-lang.github.io/rust-clippy/master/index.html#wildcard_in_or_patterns)** | complexity
0 | 21 | **[zero_ptr](https://rust-lang.github.io/rust-clippy/master/index.html#zero_ptr)** | style
5 | 15 | **[iter_nth_zero](https://rust-lang.github.io/rust-clippy/master/index.html#iter_nth_zero)** | style
12 | 8 | **[option_map_unit_fn](https://rust-lang.github.io/rust-clippy/master/index.html#option_map_unit_fn)** | complexity
5 | 15 | **[option_unwrap_used](https://rust-lang.github.io/rust-clippy/master/index.html#option_unwrap_used)** | unknown
6 | 13 | **[int_plus_one](https://rust-lang.github.io/rust-clippy/master/index.html#int_plus_one)** | complexity
7 | 12 | **[transmute_ptr_to_ref](https://rust-lang.github.io/rust-clippy/master/index.html#transmute_ptr_to_ref)** | complexity
11 | 7 | **[complexity](https://rust-lang.github.io/rust-clippy/master/index.html#complexity)** | unknown
13 | 4 | **[try_err](https://rust-lang.github.io/rust-clippy/master/index.html#try_err)** | style
8 | 8 | **[block_in_if_condition_stmt](https://rust-lang.github.io/rust-clippy/master/index.html#block_in_if_condition_stmt)** | unknown
5 | 11 | **[cmp_owned](https://rust-lang.github.io/rust-clippy/master/index.html#cmp_owned)** | perf
12 | 4 | **[match_ref_pats](https://rust-lang.github.io/rust-clippy/master/index.html#match_ref_pats)** | style
~*11*~ | ~*5*~ | ~*[verbose_bit_mask](https://rust-lang.github.io/rust-clippy/master/index.html#verbose_bit_mask)*~ | ~*style*~ pedantic
3 | 12 | **[collapsible_match](https://rust-lang.github.io/rust-clippy/master/index.html#collapsible_match)** | style
4 | 11 | **[result_unwrap_used](https://rust-lang.github.io/rust-clippy/master/index.html#result_unwrap_used)** | unknown
14 | 1 | **[style](https://rust-lang.github.io/rust-clippy/master/index.html#style)** | unknown
~*14*~ | ~*0*~ | ~*[cargo_common_metadata](https://rust-lang.github.io/rust-clippy/master/index.html#cargo_common_metadata)*~ | cargo
~*7*~ | ~*7*~ | ~*[future_not_send](https://rust-lang.github.io/rust-clippy/master/index.html#future_not_send)*~ | nursery
7 | 7 | **[just_underscores_and_digits](https://rust-lang.github.io/rust-clippy/master/index.html#just_underscores_and_digits)** | style
6 | 8 | **[needless_borrow](https://rust-lang.github.io/rust-clippy/master/index.html#needless_borrow)** | style
2 | 12 | **[option_as_ref_deref](https://rust-lang.github.io/rust-clippy/master/index.html#option_as_ref_deref)** | complexity
3 | 11 | **[redundant_allocation](https://rust-lang.github.io/rust-clippy/master/index.html#redundant_allocation)** | perf
0 | 14 | **[stable_sort_primitive](https://rust-lang.github.io/rust-clippy/master/index.html#stable_sort_primitive)** | perf
3 | 11 | **[while_let_loop](https://rust-lang.github.io/rust-clippy/master/index.html#while_let_loop)** | complexity
6 | 7 | **[bool_assert_comparison](https://rust-lang.github.io/rust-clippy/master/index.html#bool_assert_comparison)** | style
4 | 9 | **[identity_conversion](https://rust-lang.github.io/rust-clippy/master/index.html#identity_conversion)** | unknown
~*1*~ | ~*12*~ | ~*[mem_forget](https://rust-lang.github.io/rust-clippy/master/index.html#mem_forget)*~ | restriction
5 | 8 | **[useless_vec](https://rust-lang.github.io/rust-clippy/master/index.html#useless_vec)** | perf
~*4*~ | ~*8*~ | ~*[await_holding_refcell_ref](https://rust-lang.github.io/rust-clippy/master/index.html#await_holding_refcell_ref)*~ | ~*correctness*~ pedantic
1 | 11 | **[box_vec](https://rust-lang.github.io/rust-clippy/master/index.html#box_vec)** | perf
~*9*~ | ~*3*~ | ~*[from_iter_instead_of_collect](https://rust-lang.github.io/rust-clippy/master/index.html#from_iter_instead_of_collect)*~ | pedantic
2 | 10 | **[manual_filter_map](https://rust-lang.github.io/rust-clippy/master/index.html#manual_filter_map)** | complexity
5 | 7 | **[stutter](https://rust-lang.github.io/rust-clippy/master/index.html#stutter)** | unknown
7 | 5 | **[unused_io_amount](https://rust-lang.github.io/rust-clippy/master/index.html#unused_io_amount)** | correctness
1 | 10 | **[mem_discriminant_non_enum](https://rust-lang.github.io/rust-clippy/master/index.html#mem_discriminant_non_enum)** | correctness
~*7*~ | ~*4*~ | ~*[ptr_as_ptr](https://rust-lang.github.io/rust-clippy/master/index.html#ptr_as_ptr)*~ | pedantic
3 | 8 | **[unnecessary_mut_passed](https://rust-lang.github.io/rust-clippy/master/index.html#unnecessary_mut_passed)** | style
2 | 9 | **[vtable_address_comparisons](https://rust-lang.github.io/rust-clippy/master/index.html#vtable_address_comparisons)** | correctness
~*9*~ | ~*1*~ | ~*[checked_conversions](https://rust-lang.github.io/rust-clippy/master/index.html#checked_conversions)*~ | pedantic
~*3*~ | ~*7*~ | ~*[debug_assert_with_mut_call](https://rust-lang.github.io/rust-clippy/master/index.html#debug_assert_with_mut_call)*~ | nursery
1 | 9 | **[drop_ref](https://rust-lang.github.io/rust-clippy/master/index.html#drop_ref)** | correctness
0 | 10 | **[filter_next](https://rust-lang.github.io/rust-clippy/master/index.html#filter_next)** | complexity
0 | 10 | **[manual_flatten](https://rust-lang.github.io/rust-clippy/master/index.html#manual_flatten)** | complexity
~*10*~ | ~*0*~ | ~*[multiple_inherent_impl](https://rust-lang.github.io/rust-clippy/master/index.html#multiple_inherent_impl)*~ | restriction
9 | 1 | **[nursery](https://rust-lang.github.io/rust-clippy/master/index.html#nursery)** | unknown
~*4*~ | ~*6*~ | ~*[redundant_closure_for_method_calls](https://rust-lang.github.io/rust-clippy/master/index.html#redundant_closure_for_method_calls)*~ | pedantic
6 | 4 | **[redundant_slicing](https://rust-lang.github.io/rust-clippy/master/index.html#redundant_slicing)** | complexity
~*4*~ | ~*6*~ | ~*[unsafe_derive_deserialize](https://rust-lang.github.io/rust-clippy/master/index.html#unsafe_derive_deserialize)*~ | pedantic
~*6*~ | ~*4*~ | ~*[use_debug](https://rust-lang.github.io/rust-clippy/master/index.html#use_debug)*~ | restriction
~*9*~ | ~*0*~ | ~*[clone_on_ref_ptr](https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_ref_ptr)*~ | restriction
0 | 9 | **[forget_ref](https://rust-lang.github.io/rust-clippy/master/index.html#forget_ref)** | correctness
1 | 8 | **[match_overlapping_arm](https://rust-lang.github.io/rust-clippy/master/index.html#match_overlapping_arm)** | style
~*7*~ | ~*2*~ | ~*[panic_in_result_fn](https://rust-lang.github.io/rust-clippy/master/index.html#panic_in_result_fn)*~ | restriction
3 | 6 | **[single_char_pattern](https://rust-lang.github.io/rust-clippy/master/index.html#single_char_pattern)** | perf
~*7*~ | ~*1*~ | ~*[decimal_literal_representation](https://rust-lang.github.io/rust-clippy/master/index.html#decimal_literal_representation)*~ | restriction
~*7*~ | ~*1*~ | ~*[explicit_iter_loop](https://rust-lang.github.io/rust-clippy/master/index.html#explicit_iter_loop)*~ | pedantic
1 | 7 | **[fn_address_comparisons](https://rust-lang.github.io/rust-clippy/master/index.html#fn_address_comparisons)** | correctness
0 | 8 | **[infallible_destructuring_match](https://rust-lang.github.io/rust-clippy/master/index.html#infallible_destructuring_match)** | style
2 | 6 | **[manual_memcpy](https://rust-lang.github.io/rust-clippy/master/index.html#manual_memcpy)** | perf
~*8*~ | ~*0*~ | ~*[match_on_vec_items](https://rust-lang.github.io/rust-clippy/master/index.html#match_on_vec_items)*~ | pedantic
~*8*~ | ~*0*~ | ~*[modulo_arithmetic](https://rust-lang.github.io/rust-clippy/master/index.html#modulo_arithmetic)*~ | restriction
~*0*~ | ~*8*~ | ~*[needless_continue](https://rust-lang.github.io/rust-clippy/master/index.html#needless_continue)*~ | pedantic
6 | 2 | **[precedence](https://rust-lang.github.io/rust-clippy/master/index.html#precedence)** | complexity
0 | 8 | **[size_of_in_element_count](https://rust-lang.github.io/rust-clippy/master/index.html#size_of_in_element_count)** | correctness
~*3*~ | ~*5*~ | ~*[unused_async](https://rust-lang.github.io/rust-clippy/master/index.html#unused_async)*~ | pedantic
4 | 4 | **[write_literal](https://rust-lang.github.io/rust-clippy/master/index.html#write_literal)** | style
~*1*~ | ~*6*~ | ~*[await_holding_lock](https://rust-lang.github.io/rust-clippy/master/index.html#await_holding_lock)*~ | ~*correctness*~ pedantic
~*7*~ | ~*0*~ | ~*[default_numeric_fallback](https://rust-lang.github.io/rust-clippy/master/index.html#default_numeric_fallback)*~ | restriction
~*7*~ | ~*0*~ | ~*[expl_impl_clone_on_copy](https://rust-lang.github.io/rust-clippy/master/index.html#expl_impl_clone_on_copy)*~ | pedantic
~*1*~ | ~*6*~ | ~*[explicit_deref_methods](https://rust-lang.github.io/rust-clippy/master/index.html#explicit_deref_methods)*~ | pedantic
0 | 7 | **[inherent_to_string_shadow_display](https://rust-lang.github.io/rust-clippy/master/index.html#inherent_to_string_shadow_display)** | correctness
1 | 6 | **[neg_multiply](https://rust-lang.github.io/rust-clippy/master/index.html#neg_multiply)** | style
3 | 4 | **[new_without_default_derive](https://rust-lang.github.io/rust-clippy/master/index.html#new_without_default_derive)** | unknown
7 | 0 | **[option_map_unwrap_or](https://rust-lang.github.io/rust-clippy/master/index.html#option_map_unwrap_or)** | unknown
7 | 0 | **[option_map_unwrap_or_else](https://rust-lang.github.io/rust-clippy/master/index.html#option_map_unwrap_or_else)** | unknown
0 | 7 | **[unnecessary_sort_by](https://rust-lang.github.io/rust-clippy/master/index.html#unnecessary_sort_by)** | complexity
~*2*~ | ~*5*~ | ~*[unwrap_in_result](https://rust-lang.github.io/rust-clippy/master/index.html#unwrap_in_result)*~ | restriction
6 | 1 | **[useless_format](https://rust-lang.github.io/rust-clippy/master/index.html#useless_format)** | complexity
1 | 5 | **[double_parens](https://rust-lang.github.io/rust-clippy/master/index.html#double_parens)** | complexity
~*2*~ | ~*4*~ | ~*[get_unwrap](https://rust-lang.github.io/rust-clippy/master/index.html#get_unwrap)*~ | restriction
1 | 5 | **[manual_swap](https://rust-lang.github.io/rust-clippy/master/index.html#manual_swap)** | complexity
~*6*~ | ~*0*~ | ~*[map_unwrap_or](https://rust-lang.github.io/rust-clippy/master/index.html#map_unwrap_or)*~ | pedantic
5 | 1 | **[mem_replace_with_default](https://rust-lang.github.io/rust-clippy/master/index.html#mem_replace_with_default)** | style
5 | 1 | **[option_expect_used](https://rust-lang.github.io/rust-clippy/master/index.html#option_expect_used)** | unknown
5 | 1 | **[result_expect_used](https://rust-lang.github.io/rust-clippy/master/index.html#result_expect_used)** | unknown
4 | 1 | **[cargo](https://rust-lang.github.io/rust-clippy/master/index.html#cargo)** | unknown
3 | 2 | **[enum_clike_unportable_variant](https://rust-lang.github.io/rust-clippy/master/index.html#enum_clike_unportable_variant)** | correctness
~*2*~ | ~*3*~ | ~*[exit](https://rust-lang.github.io/rust-clippy/master/index.html#exit)*~ | restriction
1 | 4 | **[into_iter_on_array](https://rust-lang.github.io/rust-clippy/master/index.html#into_iter_on_array)** | unknown
2 | 3 | **[manual_async_fn](https://rust-lang.github.io/rust-clippy/master/index.html#manual_async_fn)** | style
~*0*~ | ~*5*~ | ~*[maybe_infinite_iter](https://rust-lang.github.io/rust-clippy/master/index.html#maybe_infinite_iter)*~ | pedantic
1 | 4 | **[question_mark](https://rust-lang.github.io/rust-clippy/master/index.html#question_mark)** | style
1 | 4 | **[unnecessary_filter_map](https://rust-lang.github.io/rust-clippy/master/index.html#unnecessary_filter_map)** | complexity
~*5*~ | ~*0*~ | ~*[wildcard_dependencies](https://rust-lang.github.io/rust-clippy/master/index.html#wildcard_dependencies)*~ | cargo
0 | 5 | **[zero_divided_by_zero](https://rust-lang.github.io/rust-clippy/master/index.html#zero_divided_by_zero)** | complexity
4 | 0 | **[find_map](https://rust-lang.github.io/rust-clippy/master/index.html#find_map)** | deprecated
2 | 2 | **[float_equality_without_abs](https://rust-lang.github.io/rust-clippy/master/index.html#float_equality_without_abs)** | suspicious
~*2*~ | ~*2*~ | ~*[inefficient_to_string](https://rust-lang.github.io/rust-clippy/master/index.html#inefficient_to_string)*~ | ~*perf*~ pedantic
1 | 3 | **[manual_unwrap_or](https://rust-lang.github.io/rust-clippy/master/index.html#manual_unwrap_or)** | complexity
0 | 4 | **[mut_range_bound](https://rust-lang.github.io/rust-clippy/master/index.html#mut_range_bound)** | suspicious
~*1*~ | ~*3*~ | ~*[naive_bytecount](https://rust-lang.github.io/rust-clippy/master/index.html#naive_bytecount)*~ | pedantic
4 | 0 | **[needless_arbitrary_self_type](https://rust-lang.github.io/rust-clippy/master/index.html#needless_arbitrary_self_type)** | complexity
3 | 1 | **[needless_question_mark](https://rust-lang.github.io/rust-clippy/master/index.html#needless_question_mark)** | complexity
0 | 3 | **[bad_bit_mask](https://rust-lang.github.io/rust-clippy/master/index.html#bad_bit_mask)** | correctness
~*0*~ | ~*3*~ | ~*[case_sensitive_file_extension_comparisons](https://rust-lang.github.io/rust-clippy/master/index.html#case_sensitive_file_extension_comparisons)*~ | pedantic
3 | 0 | **[cmp_null](https://rust-lang.github.io/rust-clippy/master/index.html#cmp_null)** | style
2 | 1 | **[comparison_to_empty](https://rust-lang.github.io/rust-clippy/master/index.html#comparison_to_empty)** | style
2 | 1 | **[crosspointer_transmute](https://rust-lang.github.io/rust-clippy/master/index.html#crosspointer_transmute)** | complexity
~*0*~ | ~*3*~ | ~*[disallowed_method](https://rust-lang.github.io/rust-clippy/master/index.html#disallowed_method)*~ | nursery
~*2*~ | ~*1*~ | ~*[explicit_into_iter_loop](https://rust-lang.github.io/rust-clippy/master/index.html#explicit_into_iter_loop)*~ | pedantic
~*3*~ | ~*0*~ | ~*[float_cmp_const](https://rust-lang.github.io/rust-clippy/master/index.html#float_cmp_const)*~ | restriction
0 | 3 | **[map_identity](https://rust-lang.github.io/rust-clippy/master/index.html#map_identity)** | complexity
0 | 3 | **[ok_expect](https://rust-lang.github.io/rust-clippy/master/index.html#ok_expect)** | style
3 | 0 | **[perf](https://rust-lang.github.io/rust-clippy/master/index.html#perf)** | unknown
3 | 0 | **[possible_missing_comma](https://rust-lang.github.io/rust-clippy/master/index.html#possible_missing_comma)** | correctness
~*1*~ | ~*2*~ | ~*[range_minus_one](https://rust-lang.github.io/rust-clippy/master/index.html#range_minus_one)*~ | pedantic
~*3*~ | ~*0*~ | ~*[shadow_same](https://rust-lang.github.io/rust-clippy/master/index.html#shadow_same)*~ | restriction
1 | 2 | **[single_char_add_str](https://rust-lang.github.io/rust-clippy/master/index.html#single_char_add_str)** | style
0 | 3 | **[while_immutable_condition](https://rust-lang.github.io/rust-clippy/master/index.html#while_immutable_condition)** | correctness
0 | 2 | **[char_lit_as_u8](https://rust-lang.github.io/rust-clippy/master/index.html#char_lit_as_u8)** | complexity
2 | 0 | **[const_static_lifetime](https://rust-lang.github.io/rust-clippy/master/index.html#const_static_lifetime)** | unknown
2 | 0 | **[diverging_sub_expression](https://rust-lang.github.io/rust-clippy/master/index.html#diverging_sub_expression)** | complexity
2 | 0 | **[double_must_use](https://rust-lang.github.io/rust-clippy/master/index.html#double_must_use)** | style
~*0*~ | ~*2*~ | ~*[empty_line_after_outer_attr](https://rust-lang.github.io/rust-clippy/master/index.html#empty_line_after_outer_attr)*~ | nursery
0 | 2 | **[for_kv_map](https://rust-lang.github.io/rust-clippy/master/index.html#for_kv_map)** | style
0 | 2 | **[for_loops_over_fallibles](https://rust-lang.github.io/rust-clippy/master/index.html#for_loops_over_fallibles)** | suspicious
0 | 2 | **[from_str_radix_10](https://rust-lang.github.io/rust-clippy/master/index.html#from_str_radix_10)** | style
1 | 1 | **[invalid_regex](https://rust-lang.github.io/rust-clippy/master/index.html#invalid_regex)** | correctness
0 | 2 | **[invisible_characters](https://rust-lang.github.io/rust-clippy/master/index.html#invisible_characters)** | correctness
0 | 2 | **[let_underscore_lock](https://rust-lang.github.io/rust-clippy/master/index.html#let_underscore_lock)** | correctness
0 | 2 | **[logic_bug](https://rust-lang.github.io/rust-clippy/master/index.html#logic_bug)** | correctness
1 | 1 | **[manual_find_map](https://rust-lang.github.io/rust-clippy/master/index.html#manual_find_map)** | complexity
1 | 1 | **[match_as_ref](https://rust-lang.github.io/rust-clippy/master/index.html#match_as_ref)** | complexity
0 | 2 | **[mistyped_literal_suffixes](https://rust-lang.github.io/rust-clippy/master/index.html#mistyped_literal_suffixes)** | correctness
2 | 0 | **[mixed_case_hex_literals](https://rust-lang.github.io/rust-clippy/master/index.html#mixed_case_hex_literals)** | style
0 | 2 | **[modulo_one](https://rust-lang.github.io/rust-clippy/master/index.html#modulo_one)** | correctness
~*2*~ | ~*0*~ | ~*[needless_for_each](https://rust-lang.github.io/rust-clippy/master/index.html#needless_for_each)*~ | pedantic
0 | 2 | **[out_of_bounds_indexing](https://rust-lang.github.io/rust-clippy/master/index.html#out_of_bounds_indexing)** | correctness
2 | 0 | **[panicking_unwrap](https://rust-lang.github.io/rust-clippy/master/index.html#panicking_unwrap)** | correctness
0 | 2 | **[print_with_newline](https://rust-lang.github.io/rust-clippy/master/index.html#print_with_newline)** | style
1 | 1 | **[redundant_pattern](https://rust-lang.github.io/rust-clippy/master/index.html#redundant_pattern)** | style
2 | 0 | **[restriction](https://rust-lang.github.io/rust-clippy/master/index.html#restriction)** | unknown
~*1*~ | ~*1*~ | ~*[same_functions_in_if_condition](https://rust-lang.github.io/rust-clippy/master/index.html#same_functions_in_if_condition)*~ | pedantic
~*2*~ | ~*0*~ | ~*[string_add](https://rust-lang.github.io/rust-clippy/master/index.html#string_add)*~ | restriction
0 | 2 | **[string_extend_chars](https://rust-lang.github.io/rust-clippy/master/index.html#string_extend_chars)** | style
2 | 0 | **[suspicious_assignment_formatting](https://rust-lang.github.io/rust-clippy/master/index.html#suspicious_assignment_formatting)** | suspicious
0 | 2 | **[transmuting_null](https://rust-lang.github.io/rust-clippy/master/index.html#transmuting_null)** | correctness
0 | 2 | **[unnecessary_fold](https://rust-lang.github.io/rust-clippy/master/index.html#unnecessary_fold)** | style
0 | 1 | **[as_conversion](https://rust-lang.github.io/rust-clippy/master/index.html#as_conversion)** | unknown
0 | 1 | **[async_yields_async](https://rust-lang.github.io/rust-clippy/master/index.html#async_yields_async)** | correctness
~*1*~ | ~*0*~ | ~*[cloned_instead_of_copied](https://rust-lang.github.io/rust-clippy/master/index.html#cloned_instead_of_copied)*~ | pedantic
0 | 1 | **[cmp_nan](https://rust-lang.github.io/rust-clippy/master/index.html#cmp_nan)** | correctness
~*1*~ | ~*0*~ | ~*[copy_iterator](https://rust-lang.github.io/rust-clippy/master/index.html#copy_iterator)*~ | pedantic
~*1*~ | ~*0*~ | ~*[create_dir](https://rust-lang.github.io/rust-clippy/master/index.html#create_dir)*~ | restriction
1 | 0 | **[deprecated_semver](https://rust-lang.github.io/rust-clippy/master/index.html#deprecated_semver)** | correctness
0 | 1 | **[explicit_write](https://rust-lang.github.io/rust-clippy/master/index.html#explicit_write)** | complexity
0 | 1 | **[extend_from_slice](https://rust-lang.github.io/rust-clippy/master/index.html#extend_from_slice)** | deprecated
~*0*~ | ~*1*~ | ~*[filetype_is_file](https://rust-lang.github.io/rust-clippy/master/index.html#filetype_is_file)*~ | restriction
0 | 1 | **[filter_map_identity](https://rust-lang.github.io/rust-clippy/master/index.html#filter_map_identity)** | complexity
~*0*~ | ~*1*~ | ~*[fn_params_excessive_bools](https://rust-lang.github.io/rust-clippy/master/index.html#fn_params_excessive_bools)*~ | pedantic
0 | 1 | **[if_let_some_result](https://rust-lang.github.io/rust-clippy/master/index.html#if_let_some_result)** | style
~*1*~ | ~*0*~ | ~*[if_then_some_else_none](https://rust-lang.github.io/rust-clippy/master/index.html#if_then_some_else_none)*~ | restriction
0 | 1 | **[ifs_same_cond](https://rust-lang.github.io/rust-clippy/master/index.html#ifs_same_cond)** | correctness
~*1*~ | ~*0*~ | ~*[inconsistent_struct_constructor](https://rust-lang.github.io/rust-clippy/master/index.html#inconsistent_struct_constructor)*~ | pedantic
0 | 1 | **[infinite_iter](https://rust-lang.github.io/rust-clippy/master/index.html#infinite_iter)** | correctness
~*1*~ | ~*0*~ | ~*[inline_asm_x86_intel_syntax](https://rust-lang.github.io/rust-clippy/master/index.html#inline_asm_x86_intel_syntax)*~ | restriction
0 | 1 | **[inline_fn_without_body](https://rust-lang.github.io/rust-clippy/master/index.html#inline_fn_without_body)** | correctness
0 | 1 | **[inspect_for_each](https://rust-lang.github.io/rust-clippy/master/index.html#inspect_for_each)** | complexity
0 | 1 | **[invalid_ref](https://rust-lang.github.io/rust-clippy/master/index.html#invalid_ref)** | unknown
1 | 0 | **[iter_count](https://rust-lang.github.io/rust-clippy/master/index.html#iter_count)** | complexity
0 | 1 | **[iter_nth](https://rust-lang.github.io/rust-clippy/master/index.html#iter_nth)** | perf
1 | 0 | **[manual_saturating_arithmetic](https://rust-lang.github.io/rust-clippy/master/index.html#manual_saturating_arithmetic)** | style
~*1*~ | ~*0*~ | ~*[map_flatten](https://rust-lang.github.io/rust-clippy/master/index.html#map_flatten)*~ | pedantic
0 | 1 | **[mem_replace_option_with_none](https://rust-lang.github.io/rust-clippy/master/index.html#mem_replace_option_with_none)** | style
1 | 0 | **[mut_mutex_lock](https://rust-lang.github.io/rust-clippy/master/index.html#mut_mutex_lock)** | style
0 | 1 | **[option_env_unwrap](https://rust-lang.github.io/rust-clippy/master/index.html#option_env_unwrap)** | correctness
1 | 0 | **[panic_params](https://rust-lang.github.io/rust-clippy/master/index.html#panic_params)** | unknown
~*0*~ | ~*1*~ | ~*[print_stderr](https://rust-lang.github.io/rust-clippy/master/index.html#print_stderr)*~ | restriction
1 | 0 | **[ptr_eq](https://rust-lang.github.io/rust-clippy/master/index.html#ptr_eq)** | style
1 | 0 | **[replace_consts](https://rust-lang.github.io/rust-clippy/master/index.html#replace_consts)** | deprecated
1 | 0 | **[result_map_unit_fn](https://rust-lang.github.io/rust-clippy/master/index.html#result_map_unit_fn)** | complexity
0 | 1 | **[result_map_unwrap_or_else](https://rust-lang.github.io/rust-clippy/master/index.html#result_map_unwrap_or_else)** | unknown
1 | 0 | **[skip_while_next](https://rust-lang.github.io/rust-clippy/master/index.html#skip_while_next)** | complexity
1 | 0 | **[to_string_in_display](https://rust-lang.github.io/rust-clippy/master/index.html#to_string_in_display)** | correctness
~*1*~ | ~*0*~ | ~*[todo](https://rust-lang.github.io/rust-clippy/master/index.html#todo)*~ | restriction
1 | 0 | **[transmute_int_to_char](https://rust-lang.github.io/rust-clippy/master/index.html#transmute_int_to_char)** | complexity
0 | 1 | **[transmute_int_to_float](https://rust-lang.github.io/rust-clippy/master/index.html#transmute_int_to_float)** | complexity
0 | 1 | **[unnecessary_lazy_evaluations](https://rust-lang.github.io/rust-clippy/master/index.html#unnecessary_lazy_evaluations)** | style
1 | 0 | **[unsafe_removed_from_name](https://rust-lang.github.io/rust-clippy/master/index.html#unsafe_removed_from_name)** | style
0 | 1 | **[unsound_collection_transmute](https://rust-lang.github.io/rust-clippy/master/index.html#unsound_collection_transmute)** | correctness
0 | 1 | **[useless_asref](https://rust-lang.github.io/rust-clippy/master/index.html#useless_asref)** | complexity
~*0*~ | ~*1*~ | ~*[verbose_file_reads](https://rust-lang.github.io/rust-clippy/master/index.html#verbose_file_reads)*~ | restriction
1 | 0 | **[zero_width_space](https://rust-lang.github.io/rust-clippy/master/index.html#zero_width_space)** | unknown

<br>

#### License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this project by you, as defined in the Apache-2.0 license,
shall be dual licensed as above, without any additional terms or conditions.
</sub>
