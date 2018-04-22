use std::rc::Rc;

use clap;
use cursive::view::View;
use cursive::views::ViewBox;
use serde_json::value::Value;

use feeders::Feeder;
use fields::{FieldErrors, WidgetManager};
use fields;
use views;

/// Convienient wrapper around `Field<AutocompleteManager, String>`.
pub struct Autocomplete;

impl Autocomplete {
    /// Creates a new `Field<AutocompleteManager, String>`.
    pub fn new<IS: Into<String>, F: Feeder>(
        label: IS,
        feeder: F,
    //TODO::: rm it
    //) -> fields::Field<AutocompleteManager, String> {
    ) -> Field2 {
        //TODO::: rm it
        //fields::Field::new(label, AutocompleteManager(Rc::new(feeder)), "".to_string())
        let view = views::Autocomplete::new(feeder);
        Field2::new(label, AutocompleteManager::new(view))
    }
}

pub struct AutocompleteManager {
    view: Option<views::Autocomplete>,
}
impl AutocompleteManager {
    fn new(view: views::Autocomplete) -> AutocompleteManager {
        AutocompleteManager {
            view: Some(view),
        }
    }
}

impl WidgetManager for AutocompleteManager {
    fn build_widget(&self, label: &str, help: &str, initial: &str) -> ViewBox {
        let viewbox = self.build_value_view(&initial);
        fields::label_with_help_layout(viewbox, &label, &help)
    }
    fn get_value(&self, view_box: &ViewBox) -> String {
        let view_box = fields::value_view_from_layout(view_box);
        let autocomplete: &views::Autocomplete = (**view_box).as_any().downcast_ref().unwrap();
        let value = (&*(*autocomplete).get_value()).clone();
        value
    }
    fn build_value_view(&self, value: &str) -> ViewBox {
        let view = ViewBox::new(Box::new(
            //TODO::: rm it
            //views::Autocomplete::new(Rc::clone(&self.feeder)).value(value),
            views::Autocomplete::new(vec![""]).value(value),
        ));
        view
    }

    // NEW API

    fn take_view(&mut self) -> ViewBox {
        ViewBox::new(Box::new(self.view.take().unwrap()))
    }
    fn as_string(&self, view_box: &ViewBox) -> String {
        let ac: &views::Autocomplete = (**view_box).as_any().downcast_ref().unwrap();
        let value = (&*(*ac).get_value()).clone();
        value
    }
    fn set_value(&self, view_box: &mut ViewBox, value: &str) {
        let ac: &mut views::Autocomplete = (**view_box).as_any_mut().downcast_mut().unwrap();
        (*ac).set_value(value);
    }

    fn as_value(&self, view_box: &ViewBox) -> Value {
        let ac: &views::Autocomplete = (**view_box).as_any().downcast_ref().unwrap();
        let value = (&*(*ac).get_value()).clone();
        Value::String(value.to_owned())
    }
}



//TODO::: cleanups
use cursive::view::ViewWrapper;
use cursive::views::{LinearLayout, TextView, DummyView};
use validators::{Required, Validator};
//TODO::: rename to Field/Autocomplete/or whatever
//TODO::: mv Field to fields/mod.rs
/// TODO::: docs
pub struct Field2 {
    // TODO::: explain why these fields
    label: String,
    help: String,

    validators: Vec<Rc<Validator>>,
    view: LinearLayout,
    widget_manager: AutocompleteManager,
}
//TODO::: make it macro and use if for CheckboxField, TextField, etc.?
impl Field2 {
    fn new<IS: Into<String>>(label: IS, mut widget_manager: AutocompleteManager) -> Field2 {
        let label = label.into();
        let label_and_help = LinearLayout::horizontal()
            .child(TextView::new(label_padding(label.as_ref())))
            .child(DummyView)
            .child(TextView::new(""));
        let layout = LinearLayout::vertical()
                    //TODO:: label can't include separator
                    .child(label_and_help)
                    .child(widget_manager.take_view())
                    .child(TextView::new(""))
                    .child(DummyView);
        Field2 {
            label: label,
            help: "".to_string(),
            validators: vec![],
            view: layout,
            widget_manager: widget_manager,
        }
    }
    /// Sets initial value of field.
    pub fn initial<IS: Into<String>>(mut self, initial: IS) -> Self {
        let value = initial.into();
        self.widget_manager.set_value(
            // self.view_value_get_mut() // this makes borrow-checker sad
            self.view.get_child_mut(1).unwrap().as_any_mut().downcast_mut().unwrap(),
            value.as_ref());
        self
    }
    /// Sets `help` message for `field`.
    pub fn help(mut self, msg: &str) -> Self {
        self.set_help(msg);
        self
    }
    /// Append `validator`.
    pub fn validator<V: Validator + 'static>(mut self, validator: V) -> Self {
        self.validators.push(Rc::new(validator));
        self
    }
    /// Checks if Field is required
    pub fn is_required(&self) -> bool {
        //TODO:::
        //self.validators
        //    .iter()
        //    .any(|&ref x| (**x).as_any().downcast_ref::<Required>().is_some())
        true
    }
    /// Returns view responsible for storing value.
    ///
    /// Returns `ViewBox` since we don't know what `View` is injected.
    fn view_value_get(&self) -> &ViewBox {
        self.view.get_child(1).unwrap().as_any().downcast_ref().unwrap()
    }

    /// Returns mutable view responsible for storing value.
    ///
    /// Returns `ViewBox` since we don't know what `View` is injected.
    fn view_value_get_mut(&mut self) -> &mut ViewBox {
        self.view.get_child_mut(1).unwrap().as_any_mut().downcast_mut().unwrap()
    }

    /// Returns mutable view responsible for storing label.
    fn view_label_get(&self) -> &TextView {
        let label_and_help: &LinearLayout = self.view.get_child(0).unwrap().as_any().downcast_ref().unwrap();
        label_and_help.get_child(0).unwrap().as_any().downcast_ref().unwrap()
    }

    /// Gets label of the field
    pub fn get_label(&self) -> &str {
        &self.label
    }

    /// Returns mutable view responsible for storing help message.
    fn view_help_get_mut(&mut self) -> &mut TextView {
        let label_and_help: &mut LinearLayout = self.view.get_child_mut(0).unwrap().as_any_mut().downcast_mut().unwrap();
        label_and_help.get_child_mut(2).unwrap().as_any_mut().downcast_mut().unwrap()
    }

    /// Gets help of the field
    fn get_help(&self) -> &str {
        &self.help
    }

    /// Sets help message.
    pub fn set_help(&mut self, msg: &str) {
        self.help = msg.to_string();
        let text_view: &mut TextView = self.view_help_get_mut();
        text_view.set_content(msg);
    }

    /// Returns mutable view responsible for storing error message.
    fn view_error_get_mut(&mut self) -> &mut TextView {
        self.view.get_child_mut(2).unwrap().as_any_mut().downcast_mut().unwrap()
    }

    /// Sets error message.
    pub fn set_error(&mut self, msg: &str) {
        let text_view: &mut TextView = self.view_error_get_mut();
        text_view.set_content(msg);
    }

}
//TODO::: redefine FormField trait after cleanups
impl fields::FormField for Field2 {
    fn get_widget_manager(&self) -> &WidgetManager {
        //TODO::: cleanups
        &self.widget_manager
    }
    fn build_widget(&self) -> ViewBox {
        //TODO::: cleanups
        self.widget_manager
            .build_widget("", "", "")
    }

    fn show_errors(&mut self, errors: &FieldErrors) {
        // TODO: show all errors somehow
        self.set_error(errors.first().map(|e| e.as_ref()).unwrap_or(""));
    }

    /// Validates `Field`.
    fn validate(&mut self) -> Result<Value, FieldErrors> {
        let mut errors: FieldErrors = Vec::new();
        let value = self.widget_manager.as_string(self.view_value_get());
        for v in self.validators.iter() {
            if let Some(e) = v.validate(&value) {
                errors.push(e);
            }
        }
        let result = if errors.len() > 0 {
            self.show_errors(&errors);
            Err(errors)
        } else {
            // clean possibly errors from last validation
            self.show_errors(&vec!["".to_string()]);
            Ok(self.widget_manager.as_value(self.view_value_get()))
        };
        result
    }
    //TODO::: rm it
    //fn validate(&self, data: &str) -> Result<Value, FieldErrors> {
    //    //TODO::: cleanups
    //    let mut errors = FieldErrors::new();
    //    //for v in &self.validators {
    //    //    if let Some(e) = v.validate(data) {
    //    //        errors.push(e);
    //    //    }
    //    //}
    //    if errors.len() > 0 {
    //        Err(errors)
    //    } else {
    //        Ok(Value::String(data.to_owned()))
    //    }
    //}

    /// Gets label of the field
    fn get_label(&self) -> &str {
        &self.label
    }

    /// Builds [clap::Arg] needed by automatically generated [clap::App].
    ///
    /// [clap::Arg]: ../../clap/struct.Arg.html
    /// [clap::App]: ../../clap/struct.App.html
    //TODO::: make it trait?
    // TODO::: rename it: fn as_clap_arg(&self) -> clap::Arg {
    fn clap_arg(&self) -> clap::Arg {
        let (multiple, takes_value) = match self.widget_manager.as_value(self.view_value_get()) {
            Value::Number(_) => (false, true),
            Value::String(_) => (false, true),
            Value::Array(_) => (true, true),
            _ => (false, false),
        };
        //TODO::: &self.label is enough
        clap::Arg::with_name(self.get_label())
            .help(self.get_help())
            //TODO::: &self.label is enough
            .long(self.get_label())
            .required(self.is_required())
            .multiple(multiple)
            .takes_value(takes_value)
    }

    fn clap_args2str(&self, args: &clap::ArgMatches) -> String {
        //TODO::: cleanups
        args.value_of("").unwrap_or("").to_string()
    }
}
impl ViewWrapper for Field2 {
    wrap_impl!(self.view: LinearLayout);
}
fn label_padding(label: &str) -> String {
    format!("{:20}", label)
}




//impl fields::FormField for fields::Field<AutocompleteManager, String> {
//    fn get_widget_manager(&self) -> &WidgetManager {
//        &self.widget_manager
//    }
//    fn build_widget(&self) -> ViewBox {
//        self.widget_manager
//            .build_widget(&self.label, &self.help, &self.initial)
//    }
//
//    fn validate(&self, data: &str) -> Result<Value, FieldErrors> {
//        let mut errors = FieldErrors::new();
//        for v in &self.validators {
//            if let Some(e) = v.validate(data) {
//                errors.push(e);
//            }
//        }
//        if errors.len() > 0 {
//            Err(errors)
//        } else {
//            Ok(Value::String(data.to_owned()))
//        }
//    }
//
//    /// Gets label of the field
//    fn get_label(&self) -> &str {
//        &self.label
//    }
//
//    fn clap_arg(&self) -> clap::Arg {
//        clap::Arg::with_name(&self.label)
//            .help(&self.help)
//            .long(&self.label)
//            .required(self.is_required())
//            .takes_value(true)
//    }
//
//    fn clap_args2str(&self, args: &clap::ArgMatches) -> String {
//        args.value_of(&self.label).unwrap_or("").to_string()
//    }
//}

////TODO::: rm it
//impl<W: WidgetManager> fields::Field<W, String> {
//    /// Sets initial `value` of `field`.
//    pub fn initial<IS: Into<String>>(mut self, initial: IS) -> Self {
//        self.initial = initial.into();
//        self
//    }
//}
