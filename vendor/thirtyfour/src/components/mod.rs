//! # Components
//!
//! Components allow you to wrap elements or groups of elements in order to abstract the
//! element selectors and focus more on the logic and flow of your website or web app.
//!
//! This approach may be familiar to anyone who has used a
//! [Page Object Model](https://www.selenium.dev/documentation/test_practices/encouraged/page_object_models/) before.
//! However a `Component` can wrap any node in the DOM, not just "pages".
//!
//! It uses smart element resolvers that can lazily resolve elements within the component and cache them for subsequent
//! uses. You can also nest components, making them an extremely powerful feature for automating any modern web app.
//!
//! ### Example
//!
//! Given the following HTML structure:
//!
//! ```html
//! <div id="checkbox-section">
//!     <label>
//!         <input type="checkbox" id="checkbox-option-1" />
//!         Option 1
//!     </label>
//!
//!     <label>
//!         <input type="checkbox" id="checkbox-disabled" disabled />
//!         Option 2
//!     </label>
//!
//!     <label>
//!         <input type="checkbox" id="checkbox-hidden" style="display: none;" />
//!         Option 3
//!     </label>
//! </div>
//! ```
//!
//! ```ignore
//! /// This component shows how to wrap a simple web component.
//! #[derive(Debug, Clone, Component)]
//! pub struct CheckboxComponent {
//!     base: WebElement, // This is the <label> element
//!     #[by(css = "input[type='checkbox']")]
//!     input: ElementResolver<WebElement>, // This is the <input /> element
//! }
//!
//! impl CheckboxComponent {
//!     /// Return true if the checkbox is ticked.
//!     pub async fn is_ticked(&self) -> WebDriverResult<bool> {
//!         let elem = self.input.resolve().await?;
//!         let prop = elem.prop("checked").await?;
//!         Ok(prop.unwrap_or_default() == "true")
//!     }
//!
//!     /// Tick the checkbox if it is clickable and isn't already ticked.
//!     pub async fn tick(&self) -> WebDriverResult<()> {
//!         // This checks that the element is present before returning the element.
//!         // If the element had become stale, this would implicitly re-query the element.
//!         let elem = self.input.resolve_present().await?;
//!         if elem.is_clickable().await? && !self.is_ticked().await? {
//!             elem.click().await?;
//!             // Now make sure it's ticked.
//!             assert!(self.is_ticked().await?);
//!         }
//!
//!         Ok(())
//!     }
//! }
//!
//! /// This component shows how to nest components inside others.
//! #[derive(Debug, Clone, Component)]
//! pub struct CheckboxSectionComponent {
//!     base: WebElement, // This is the outer <div>
//!     #[by(tag = "label", allow_empty)]
//!     boxes: ElementResolver<Vec<CheckboxComponent>>, // ElementResolver works with Components too.
//!     // Other fields will be initialised with Default::default().
//!     my_field: bool,
//! }
//! ```
//!
//! So how do you construct a Component?
//!
//! Simple! The `Component` derive automatically implements `From<WebElement>`.
//!
//! ```ignore
//! let elem = driver.query(By::Id("checkbox-section")).await?;
//! let component = CheckboxSectionComponent::from(elem);
//!
//! // Now you can get the checkbox components easily like this.
//! let checkboxes = component.boxes.resolve().await?;
//! for checkbox in checkboxes {
//!     checkbox.tick().await?;
//! }
//! ```
//!
//! This allows you to wrap any component using `ElementResolver` to resolve elements and nested
//! components easily.
//!

/// Wrapper for `<select>` elements.
mod select;
/// Component wrappers.
mod wrapper;

pub use select::*;
pub use wrapper::*;
