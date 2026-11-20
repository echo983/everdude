# everdude

Unix UTC 与 **everdude** 时间制式的高性能双向转换工具与 `#![no_std]` 核心库。

## 时间制式定义

- **纪元（Epoch）**：`2001-01-01 00:00:00 UTC`（周一），对应 Unix 时间戳 `978,307,200` 秒（`978,307,200,000` 毫秒）。
- **标准单位（半周 $h$）**：
  $$h = \frac{\text{unixMs} - 978\,307\,200\,000}{302\,400\,000}$$
  其中 $302\,400\,000\text{ ms} = 3.5\text{ 天} = 84\text{ 小时}$。
  - **偶数整数位**对应周一 `00:00 UTC`
  - **奇数整数位**对应周四 `12:00 UTC`
- **二进制无损扩展**：任何「周 $\times 2^k$」单位之间在 IEEE 754 双精度实数（`f64`）下可无损换算。
- **底层时间**：Unix UTC，不计闰秒，不受夏令时影响。

## 构建

```bash
cargo build --release
```

产物位于 `target/release/everdude`。

## CLI 用法

支持命令行参数、标准输入管道与文件重定向，输出采用最短往返精度（shortest round-trip）`f64`：

```bash
# 正向转换（Unix -> everdude 半周值）
everdude 1795191539
echo "1795191539" | everdude
everdude < timestamps.txt

# 支持负数时间戳、小数秒，以及整数毫秒/微秒/纳秒自动识别（有歧义按秒）
everdude 978307200 978307200000 -302400 1700000000.5

# 反向转换（everdude 半周值 -> Unix 秒）
everdude -r 0 1 -1
echo "0 1 2" | everdude --reverse
```

## 作为库使用

核心库 `everdude` 为 `#![no_std]` 且零外部依赖，可在 Rust 项目中直接引用，或通过 `--features capi` 导出 C ABI：

- `from_unix_secs(f64) -> f64` / `from_unix_secs_i64(i64) -> f64`
- `from_unix_ms(i64) -> f64` / `from_unix_ms_f64(f64) -> f64`
- `from_unix_us_i128(i128) -> f64` / `from_unix_ns_i128(i128) -> f64`
- `to_unix_secs(f64) -> f64` / `to_unix_ms(f64) -> f64`

## 测试

```bash
cargo test
cargo test --features capi
```
