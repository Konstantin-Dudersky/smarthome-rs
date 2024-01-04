use leptos::*;

#[component]
pub fn Input(config: InputConfig) -> impl IntoView {
    view! {
            <div>
      <label for="price" class="block text-sm font-medium leading-6 text-gray-900">Price</label>
      <div class="relative mt-2 rounded-md shadow-sm">
        <div class="pointer-events-none absolute inset-y-0 left-0 flex items-center pl-3">
          <span class="text-gray-500 sm:text-sm">$</span>
        </div>
        <input type="text" name="price" id="price" class="block w-full rounded-md border-0 py-1.5 pl-7 pr-12 text-gray-900 ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6" placeholder="0.00" aria-describedby="price-currency"/>
        <div class="pointer-events-none absolute inset-y-0 right-0 flex items-center pr-3">
          <span class="text-gray-500 sm:text-sm" id="price-currency">{ config.value }</span>
        </div>
      </div>
    </div>
        }
}

#[derive(Default)]
pub struct InputConfig {
    pub value: Option<Box<dyn Fn() -> String>>,
}

impl InputConfig {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
    pub fn set_trail(mut self, trail: impl Fn() -> String + 'static) -> Self {
        self.value = Some(Box::new(trail));
        self
    }
}

// #[derive(Default)]
// pub struct InputConfigBuilder {
//     trail: Option<fn() -> String>,
// }

// impl InputConfigBuilder {
//     pub fn new() -> Self {
//         Self {
//             ..Default::default()
//         }
//     }

//     pub fn set_trail(mut self, trail: fn() -> String) -> Self {
//         self.trail = Some(trail);
//         self
//     }

//     pub fn build(self) -> InputConfig {
//         InputConfig { trail: self.trail }
//     }
// }
