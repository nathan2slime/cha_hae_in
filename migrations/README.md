### Migrator CLI Guide 

- **New migration**  
  Crie um novo arquivo de migração fofinho:  
  ```sh
  cargo run -- generate MIGRATION_NAME
  ```

- **Apply migrations**  
  Aplique todas as migrações pendentes e deixe o banco lindinho:  
  ```sh
  cargo run
  ```
  ```sh
  cargo run -- up
  ```

- **Apply some migrations**  
  Quer só as 10 primeiras migrações? Sem problemas!  
  ```sh
  cargo run -- up -n 10
  ```

- **Undo last migration**  
  Volte atrás e desfaça a última migração aplicada:  
  ```sh
  cargo run -- down
  ```

- **Undo some migrations**  
  Precisa desfazer as últimas 10 migrações? Fácil assim!  
  ```sh
  cargo run -- down -n 10
  ```

- **Fresh start**  
  Limpe tudo (tabelas e afins) e recomece do zero:  
  ```sh
  cargo run -- fresh
  ```

- **Refresh migrations**  
  Desfaça tudo e aplique as migrações de novo, tipo mágica:  
  ```sh
  cargo run -- refresh
  ```

- **Reset everything**  
  Desfaça absolutamente todas as migrações aplicadas:  
  ```sh
  cargo run -- reset
  ```

- **Check status**  
  Veja como estão as migrações e fique de olho no progresso:  
  ```sh
  cargo run -- status
  
