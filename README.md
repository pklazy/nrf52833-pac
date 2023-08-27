# NRF52833-PAC

<https://docs.rs/svd2rust/latest/svd2rust/>

```sh
wget https://github.com/NordicSemiconductor/nrfx/raw/master/mdk/nrf52833.svd
sed -i 's/read-writeonce/read-writeOnce/g' nrf52833.svd
svd2rust -i nrf52833.svd
mv lib.rs src/
cargo fmt
```
