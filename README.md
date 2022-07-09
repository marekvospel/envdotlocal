# envdotlocal
[![License](https://img.shields.io/github/license/marekvospel/envdotlocal)](https://github.com/marekvospel/envdotlocal)  
This project is an alternative to [dotenv](https://github.com/motdotla/dotenv) built with wasm written in rust with [pest.rs](https://pest.rs/).

## TODO
- [ ] (rs) replace `"\'"`, `"\\"` etc. with escaped versions
- [ ] (rs) return parsed key, value pairs to javascript
- [ ] (pest) ignore comments
- [ ] (js) read files, update `process.env`
