# Comandos úteis
Início
```bash
nohup ./palindromos_primos_pi > result2.log 2>&1 &
echo $! > save_pid.txt
```

Encerramento
```bash
kill -9 `cat save_pid.txt`
rm save_pid.txt
```

# Progresso
- [x] Atualizar metodo e usar o teste do crate, pois é simplesmente inaceitável esperar 7 horas por cada teste de numero primo.
- [x] Já temos de 25 digitos, a meta agora é calcular de 27 digitos.
- [x] Corrigir posicionamento, pois ele está errado e se eu não descobrir o que está errado precisarei repetir alguns arquivos.

# Resultado atual
[7331530558321238550351337 em 33044988112960](https://api.pi.delivery/v1/pi?start=33044988112960&numberOfDigits=25).
