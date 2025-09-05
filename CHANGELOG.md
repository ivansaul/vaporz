## [1.0.0-beta.2](https://github.com/ivansaul/vaporz/compare/v1.0.0-beta.1...v1.0.0-beta.2) (2025-09-05)

### Fix

* **win:** move symlink filter before par_bridge ([6e2fc3d](https://github.com/ivansaul/vaporz/commit/6e2fc3df3f5014c08637ff5273f748e99fb411ce))
* **win:** use explicit closure in filter_map to fix type mismatch ([67e2721](https://github.com/ivansaul/vaporz/commit/67e27218543424ee8d1c90236bff249b77cac0c3))

### Chore

* add more targets and artifacts to config.toml ([ac1174a](https://github.com/ivansaul/vaporz/commit/ac1174ac4e4ab70b8fa80307ddf2712eb2a916f3))
* **ci:** test on multiple OSes ([e00a566](https://github.com/ivansaul/vaporz/commit/e00a566f6d3a3a5ba95a8e94bf4c6cdaf4f86409))

### Docs

* add README file ([03bcae8](https://github.com/ivansaul/vaporz/commit/03bcae88088665307bfb484b16062ec23a807389))
* update demo URL ([4e8ed89](https://github.com/ivansaul/vaporz/commit/4e8ed8931b0cd980a47ef9f4f256d1f9616af9ba))

### Refactor

* **counter:** remove temporary counter feature ([82ef68f](https://github.com/ivansaul/vaporz/commit/82ef68f664a6c22961176a678156891e36237303))
* fix clippy warnings across all targets and features ([28e1c1f](https://github.com/ivansaul/vaporz/commit/28e1c1f2e644bb24ccd7f34747416ff94e7027de))

## 1.0.0-beta.1 (2025-09-04)

### Feature

* add metrics widget ([f9ff940](https://github.com/ivansaul/vaporz/commit/f9ff94080b00d4db6ada73fb336140f5aa866fee))
* add support for loading configuration from a TOML file ([1e7d3b2](https://github.com/ivansaul/vaporz/commit/1e7d3b2bee2eb16d47d4590d155581dae0c23498))
* add title to artifacts widget block ([26a7397](https://github.com/ivansaul/vaporz/commit/26a73973b752717039dcd2f4cf7a657d8c8f659a))
* initial commit for vaporz project ([d5da543](https://github.com/ivansaul/vaporz/commit/d5da54386ea97bc22ed9256011d67ff07f457b63))

### Fix

* skip hidden directories and symlinks from directory scan ([f797c95](https://github.com/ivansaul/vaporz/commit/f797c951ad33f5771ad9ed8ef57b5cdc64db499d))

### Chore

* add release profile optimizations ([00329cf](https://github.com/ivansaul/vaporz/commit/00329cf43949f6f58829d5bfef3cd7f51fc642e9))
* **ci:** add CI and CD workflows ([8759252](https://github.com/ivansaul/vaporz/commit/87592520b813c4a61a08924167cc7e1666e05598))
* remove Cargo.lock from .gitignore ([941bbd7](https://github.com/ivansaul/vaporz/commit/941bbd739d3b0e44a6f9d9a5580aa07d8084a4b9))
* support beta branch for CI/CD and releases ([7488516](https://github.com/ivansaul/vaporz/commit/7488516503816d9abcc786e2acb456219f2aade1))

### Style

* remove artifacts block title ([54af179](https://github.com/ivansaul/vaporz/commit/54af1793dad617c89ebe6ab9627faf9f22288add))

### Refactor

* add status column to artifacts table ([3a840ff](https://github.com/ivansaul/vaporz/commit/3a840ff5b6081d6d3bb990b0533f1d45a1a3ec9b))
* app layout constraints ([254c5b6](https://github.com/ivansaul/vaporz/commit/254c5b68b296269cfbeb3c2f35388fbb2f3f331e))
* improve artifacts scanning ([ab17270](https://github.com/ivansaul/vaporz/commit/ab1727012efecbd9f949928474fe72d98a0f63e2))
* rename Metrics to MetricsWidget ([059f765](https://github.com/ivansaul/vaporz/commit/059f765429767c29ca33bc9b073f08bb6e7da387))
* use Arc<RwLock<Vec<FolderInfo>> for thread safety ([e836ce1](https://github.com/ivansaul/vaporz/commit/e836ce11b81f2cf25206fe8bc2bab2663a56a181))
