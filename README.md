[Gazer](https://github.com/0xnicholas/gazer) originated from the early practice of using Rust to develop blockchain, and then with the exploration of the Monoli project, gradually formed into a development framework for building the Monoli L1 Chain.
Gazer adopts the architecture design similar to Substrate, and learn from Cosmos SDK in the implementation of some modules.

# 🔮 Gazer
Gazer is a modular and extensible blockchain framework built in Rust with support for the rWASM runtime and RISC-V runtime. It is a set of libraries and primitives for building Monoli Chain.

## How to Get Started
Gazer offers different options in terms of development freedom and convenience, using SDK templates or custom modules:
- The easiest way to use Gazer is to use one of the templates (`crate::monoli_sdk::templates`) and only tweak the parameters of the runtime or node. This allows you to launch a blockchain in minutes, but is limited in technical freedom.
- Developers develop customized modules by way of `craft`

> Using a Template -> Writing your own CRAFT-based Module -> Custom Node 

## Structure
Gazer contains a lot of crates,In broad terms, these crates are divided into three categories:
- `gc-*` (_Gazer-client_) crates, located under `./client` folder. These are all the crates that lead to the node software. Notable examples are `gc_network`, various consensus crates, RPC (`gc_rpc_api`) and database (`gc_client_db`).
- `gp-*` (_Gazer-primitives_) crates, located under `./primitives` folder. These are crates that facilitate both the node and the runtime, but are not opinionated about what framework is using for building the runtime(such as Monoli rWASM or RISC-V). Notable examples are `gp_api` and `gp_io`, which form the communication bridge between the node and runtime.
- `module-*` and `craft-*` crates, located under `./craft` folder. These are the crates related to CRAFT. See craft for more information.

## `gazer-primitives`
This is Gazer's base library, providing runtime-agnostic, general-purpose primitives including data structures, mathematical utilities, cryptography, Merkle trees, serialization, and other foundational components.

## CRAFT
CRAFT is the Gazer’s framework of choice to build a runtime.
CRAFT is composed of two major components, modules and a runtime.

### Modules
Modules are functional units that build the Gazer Runtime. Each module encapsulates a portion of the business logic, such as accounts, staking, gov, and so on.

A module is defined as a `mod module` wrapped by the `craft::module` macro. Within this macro, module components can be defined. Most notable of these parts are:
- Config, allowing a module to make itself configurable and generic over types, values and such.
- Storage, allowing a module to define on-chain storage.
- Dispatchable function, is as function that can be called by the end-user through a tx, which is the “executable method” exposed to the public by module.
- Events, allowing a module to emit events.
- Errors,  allowing a module to emit well-formed errors.

#### Example
The following example showcases a minimal module.
```rust
#[craft::module(dev_mode)]
pub mod module {
	use craft::prelude::*;
    
	#[module::config]
	pub trait Config: module_system::Config {
		#[allow(deprecated)]
		type RuntimeEvent: IsType<<Self as craft_system::Config>::RuntimeEvent> + From<Event<Self>>;
        
        type ValueParameter: Get<u32>;
        
		const ANOTHER_VALUE_PARAMETER: u32;
	}
    
	#[module::module]
	pub struct Module<T>(PhantomData<T>);
    
    #[module::event]
	pub enum Event<T: Config> {}
    
	#[module::storage]
	pub type Value<T> = StorageValue<Value = u32>;
    
	#[module::call]
	impl<T: Config> Module<T> {
        
		pub fn some_dispatchable(
			_origin: OriginFor<T>,
			_param: u32,
			_other_para: u32,
		) -> DispatchResult {
			Ok(())
		}
	}
}
```

### Runtime
The Runtime is the “state machine” of the blockchain and is an aggregation of modules. It defines the STF for the entire chain, i.e. how blocks, transactions, etc. are handled.

The Runtime is eventually compiled into a rWASM runtime (or RISC-V runtime) and deployed to the chain.

Runtime assembles multiple modules together, and each module usually has some configuration (exposed as a trait config) that needs to be specified in the runtime. This is done with `craft::runtime::prelude::construct_runtime`.

#### Example
The following example shows a test that is composing the module demonstrated above, next to the `craft::prelude::craft_system` module, into a runtime.
```rust
pub mod runtime {
	use super::module as module_example;
	use craft::{prelude::*, testing_prelude::*};

	construct_runtime!(
		pub enum Runtime {
			System: craft_system,
			Example: module_example,
		}
	);

	#[derive_impl(craft_system::config_modules::TestDefaultConfig)]
	impl craft_system::Config for Runtime {
		type Block = MockBlock<Self>;
	}

	impl module_example::Config for Runtime {
		type RuntimeEvent = RuntimeEvent;
		type ValueParameter = ConstU32<42>;
		const ANOTHER_VALUE_PARAMETER: u32 = 42;
	}
}
```

## Client
`gc-client` serves as the central execution engine of the node, handling block execution via the runtime, consensus integration, database management, block synchronization, and state verification.

---

## _Reference
* [Monoli](https://github.com/0xnicholas/monoli)
* [Gazer Docs](https://nicholas.feishu.cn/wiki/PN5iw3MN7iLKWVkeLpmclVaUn9g)
* [Substrate Interpretation Docs](https://nicholas.feishu.cn/wiki/BdxfwJmVPiFOWYkLkj0cBWjgnIG?from=from_copylink)
* [Using Cosmos SDK](https://nicholas.feishu.cn/wiki/OVyBwvOkEiLyZkklVYecarxan5c?from=from_copylink)
