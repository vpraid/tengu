## Tengu

Tengu is a machine learning and tensor algebra library for Rust developers.

At the base of the project lie two foundational crates: [tengu-tensor](tengu-tensor) that implements tensor storage, and [tengu-algebra](tengu-algebra) that implements low-level primitives for tensor algebra. Everything else is optional, so you can pick and choose what you need in your project:

| Crate | Description |
| --- | --- |
| [tengu-learning](tengu-learning) | Classical machine learning algorithms |
| [tengu-neural](tengu-neural) | Neural networks |
| [tengu-nlp](tengu-nlp) | Natural language processing |
| [tengu-vision](tengu-vision) | Computer vision |

## Philosophy

We have several considerations in mind when developing this project:
- It must be fast. Rust is all about speed, so it would be unwise to ignore the gift.
- It must be ergonomic. Many ML library suffer from unintuitve and bloated interfaces. We strive to avoid that and bring the clarity to you.
- It must be portable. No obscure dependencies on system libraries or binding to existing projects which may or may not work on your system.
- It must be comprehensive. Everything you have every seen in a numpy, scikit-learn, or other popular library is here.
- It must be modular. Pick the crate you need. Use only what you really need to use.

## Contributing
Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

Please make sure to update tests as appropriate.

## License
[MIT](https://choosealicense.com/licenses/mit/)
