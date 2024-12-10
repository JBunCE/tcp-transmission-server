# TCP Transmission Server

Este proyecto es un servidor de transmisión TCP desarrollado en Rust usando Tokio y otros módulos personalizados. Su objetivo es manejar conexiones TCP de manera asíncrona, permitiendo la transmisión de señales y eventos de manera eficiente.

## Tecnologías
- **Rust**
- **Tokio** (para manejo de tareas asíncronas y sockets TCP)

## Requisitos
- **Rust** (instalado desde [rustup](https://rustup.rs/))
- **Cargo** (gestor de paquetes y compilador de Rust)

## Puesta en marcha
### Instalación de Rust
Si aún no tienes Rust instalado, sigue estos pasos:
1. Descarga e instala Rust ejecutando el siguiente comando:
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```
2. Sigue las instrucciones en pantalla para completar la instalación.
3. Verifica que Rust está instalado ejecutando:
   ```bash
   rustc --version
   ```

### Clonar el repositorio
Clona este repositorio en tu máquina local usando el siguiente comando:
```bash
git clone <URL-DEL-REPOSITORIO>
cd tcp-transmission-server
```

### Ejecutar el servidor
Para iniciar el servidor, simplemente ejecuta:
```bash
cargo run
```
El servidor estará escuchando en la dirección `0.0.0.0` y el puerto `1998`.

## Funcionamiento
El archivo principal del servidor es `main.rs`. Su funcionamiento básico incluye:
- Escuchar conexiones TCP entrantes en el puerto 1998.
- Manejar clientes y streamers de forma asíncrona.
- Transmitir señales y eventos usando un canal Tokio (`mpsc`).

### Arquitectura
- **models**: Contiene las definiciones de eventos y señales del servidor.
- **utils**: Funciones de utilidad para tareas comunes.
- **connection_handler**: Maneja las conexiones entrantes.
- **stream_handler**: Maneja el flujo de datos de los streamers.
- **broker**: Responsable de producir eventos.

### Configuración de red
Si necesitas cambiar la dirección o el puerto del servidor, modifica las constantes `ADDRESS` y `PORT` en `main.rs`:
```rust
const PORT: i32 = 1998;
const ADDRESS: &str = "0.0.0.0";
```

## Documentación de la API
Este proyecto utiliza señales y eventos definidos en los módulos `models::server_signals` y `models::server_events`. Asegúrate de explorar estos archivos para entender cómo interactuar con el servidor.

## Notas finales
- Este servidor está diseñado para ser extendido fácilmente. Puedes agregar nuevos módulos o mejorar los existentes según sea necesario.
- Si encuentras algún problema, por favor abre un issue en el repositorio.

