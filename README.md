# A python interface for drand-verify

## Based on [drand-verify](https://github.com/noislabs/drand-verify)
### Made with [PyO3](https://github.com/PyO3/pyo3)

## Usage

#### Returns the randomness for the given round if the input is valid.
#### Attempting to verify an invalid/incorrect input will raise a `ValueError`.

```py
import drand_verify as drv

#                  round,   previous signature,                                                                                                                                                                                 signature,                                                                                                                                                                                        public key (optional, defaults to League of Entropy mainnet key)
drv.verify_mainnet(3311596, "8ed588f2a7716fb1349e2d9803da5db0005e98a83783c353d4a08f183236a9ad91d70ddb01266f4b7c576983db464b430e65680b9e0098552758afd6c1e6afcb77e3f62fe1b93d42d1cb63abbb2205512fe12fbf74ea9c5ac3b8f5c1e283a1d8", "a696b9409ababce45749c3a4ec369074453dd4a79967734e1390d969c8ad8d98897d217b9121e92c8ddebbddda8d92f900e3bd6bf9deb166863b1a19390d743f82774001487594c5c09e581db7365f02b70a2c8cc41ce32446ef08e4890c4754")
>>> '647c07b2abbf7ff2afb4d670214f565e5cd9f9c91bfdcecb59a21f3c78d73920'

#                   round,  signature,                                                                                        public key (optional, defaults to League of Entropy quicknet key)
drv.verify_quicknet(657413, "b713718a38ae728dfd477991af2822e08d2f305e47718cef9f7848ce4050e7be41076862b98fad56e91a6b85b89cd97b")
>>> 'fc1873a13f3545aeade8401532ef5519920652eee6b0d2b19ca12643b87b3587'
```
