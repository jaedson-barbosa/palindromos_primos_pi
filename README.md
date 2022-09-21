nohup ./palindromos_primos_pi > result2.log 2>&1 &
echo $! > save_pid.txt

kill -9 `cat save_pid.txt`
rm save_pid.txt

- [x] Atualizar metodo e usar o teste do crate, pois é simplesmente inaceitável esperar 7 horas por cada teste de numero primo.
- [ ] Já temos de 25 digitos, a meta agora é calcular de 27 digitos.
- [ ] Corrigir posicionamento, pois ele está errado e se eu não descobrir o que está errado precisarei repetir alguns arquivos.
