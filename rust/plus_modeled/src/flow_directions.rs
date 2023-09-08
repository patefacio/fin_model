//! Contains static mapping of flow direction of all flow categories.

////////////////////////////////////////////////////////////////////////////////////
// --- module uses ---
////////////////////////////////////////////////////////////////////////////////////
use crate::FlowDirection;
use crate::FlowType;
use crate::SystemId;
use once_cell::sync::Lazy;
use std::collections::HashMap;

////////////////////////////////////////////////////////////////////////////////////
// --- lazy inits ---
////////////////////////////////////////////////////////////////////////////////////
/// Mapping of the flow system ids to the flow direction
pub static FLOW_DIRECTIONS_MAP: Lazy<HashMap<SystemId, FlowDirection>> = Lazy::new(|| {
    HashMap::from([
        /*
          Earned Income
          Money earned from a job - wages, salaries, tips etc
        */
        (SystemId::FlowItemId(0), FlowDirection::InFlow),
        /*
          Pension Income
          Money from pension
        */
        (SystemId::FlowItemId(1), FlowDirection::InFlow),
        /*
          Social Security Income
          Money from social security
        */
        (SystemId::FlowItemId(2), FlowDirection::InFlow),
        /*
          Rental Income
          Money from rental properties - passive income
        */
        (SystemId::FlowItemId(3), FlowDirection::InFlow),
        /*
          Royalty Income
          Money from royalties such as music, books, manuscripts, computer software, or a patent
        */
        (SystemId::FlowItemId(4), FlowDirection::InFlow),
        /*
          Internet Advertising Income
          Money from internet advertising - passive income
        */
        (SystemId::FlowItemId(5), FlowDirection::InFlow),
        /*
          Passive Business Income
          Money from business you are not actively involved in - passive income
        */
        (SystemId::FlowItemId(6), FlowDirection::InFlow),
        /*
          Ordinary Income
          Money classified as ordinary income for tax purposes
        */
        (SystemId::FlowItemId(7), FlowDirection::InFlow),
        /*
          Living Expense
          General cost of living
        */
        (SystemId::FlowItemId(8), FlowDirection::OutFlow),
        /*
          Health Care Expense
          Costs associated with health care
        */
        (SystemId::FlowItemId(9), FlowDirection::OutFlow),
        /*
          College Expense
          Cost of college which historically has grown faster than inflation
        */
        (SystemId::FlowItemId(10), FlowDirection::OutFlow),
        /*
          Property Taxes
          Property taxes, which may be deductible.
          Property taxes may be deductible if you itemize, but a limit comes into play.

          Under a massive tax overhaul that was signed into law in 2017, deductible state and local income taxes
          (SALT), including property taxes, are capped at $10,000.

          The limit is scheduled to last through the 2025 tax year, unless Congress extends it.
        */
        (SystemId::FlowItemId(11), FlowDirection::OutFlow),
        /*
          Mortgage Interest
          Mortgage interest, which may be deductible.
          From [_Forbe's_ article](https://www.forbes.com/advisor/taxes/tax-write-offs-you-can-claim-on-your-taxes/)
          The interest you pay for your mortgage can be deducted from your taxes. The write-off is limited to interest
          on up to $750,000 ($375,000 for married-filing-separately taxpayers) of mortgage debt incurred
          after Dec. 15, 2017.
        */
        (SystemId::FlowItemId(12), FlowDirection::OutFlow),
        /*
          State Taxes Paid
          State taxes paid, which may be deductible.
          you can deduct state income taxes that are paid, but the write-off is limited to up to $10,000,
          which includes all deductible state and local taxes.
        */
        (SystemId::FlowItemId(13), FlowDirection::OutFlow),
        /*
          Charitable Donations
          Charitable donations, which may be deductible.
          From [_Forbe's_ article](https://www.forbes.com/advisor/taxes/tax-write-offs-you-can-claim-on-your-taxes/)
          Generally, you can deduct charitable contributions of cash totaling up to 60% of your adjusted gross income,
          or AGI. Donations of items or property also are considered deductible charitable contributions.
        */
        (SystemId::FlowItemId(14), FlowDirection::OutFlow),
        /*
          Medical Expenses
          Medical expenses, which may be deductible.
          From [_Forbe's_ article](https://www.forbes.com/advisor/taxes/tax-write-offs-you-can-claim-on-your-taxes/)
          Medical and dental expenses qualify for a tax deduction, though you can deduct only the costs that exceed
          7.5% of your AGI.
          To claim medical-related expenses on your 2022 tax return next year, they must have been paid in 2022, unless
          they were charged to a credit card. In those cases, you can deduct the expenses in the year you charged the
          card, not necessarily the year in which you repaid them.

          Trips to your doctor’s office or hospital appointments qualify for medical mileage. For 2022, you can deduct
          18 cents a mile for travel you made for medical purposes through June 2022. The amount has increased to 22
          cents a mile from July 1, 2022, through the end of the year.
        */
        (SystemId::FlowItemId(15), FlowDirection::OutFlow),
        /*
          Retirement Credits
          Retirement credits, which may be deductible.
          From [_Forbe's_ article](https://www.forbes.com/advisor/taxes/tax-write-offs-you-can-claim-on-your-taxes/)
          The contributions you make to a retirement plan such as a 401(k) or a traditional or Roth IRA give you a tax
          credit of 50%, 20% or 10%, depending on your adjusted gross income that you report on Form 1040. Any rollover
          contributions do not qualify for the credit.

          The maximum contribution amount that qualifies for the credit is $2,000 ($4,000 if married filing
          jointly), making the maximum possible credit $1,000 ($2,000 if married filing jointly). The IRS has
          a chart to help calculate your credit.
        */
        (SystemId::FlowItemId(16), FlowDirection::OutFlow),
        /*
          Ira Contributions
          IRA contributions, which may be deductible.
          From [_Forbe's_ article](https://www.forbes.com/advisor/taxes/tax-write-offs-you-can-claim-on-your-taxes/)
          The maximum contribution for 2022 in a traditional or Roth IRA is $6,000, plus another $1,000 for
          people who are 50 years old or more. Your contributions to a traditional IRA are tax-deductible.
        */
        (SystemId::FlowItemId(17), FlowDirection::OutFlow),
        /*
          Other In Flow
          An _atypical_ in flow not represented by other categories
        */
        (SystemId::FlowItemId(18), FlowDirection::InFlow),
        /*
          Other Out Flow
          An _atypical_ out flow not represented by other categories
        */
        (SystemId::FlowItemId(19), FlowDirection::OutFlow),
    ])
});

////////////////////////////////////////////////////////////////////////////////////
// --- functions ---
////////////////////////////////////////////////////////////////////////////////////
/// Get the direction of the flow from type
///
///   * **flow_type** - Type of flow
///   * _return_ - Direction (In/Out/...) of flow
#[inline]
pub fn get_flow_direction(flow_type: &FlowType) -> FlowDirection {
    // α <fn get_flow_direction>

    let system_id = SystemId::FlowItemId(*flow_type as u32);

    FLOW_DIRECTIONS_MAP
        .get(&system_id)
        .cloned()
        .unwrap_or(FlowDirection::InFlow)
    // ω <fn get_flow_direction>
}

// α <mod-def flow_directions>
// ω <mod-def flow_directions>
