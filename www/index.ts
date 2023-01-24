import init, {Direction, World} from 'snake_game';
import {rnd} from "./utils/rnd";

init().then(wasm => {
  const FPS = 5;
  const CELL_SIZE = 20;
  const WORLD_WIDTH = 8;
  const WORLD_SIZE = WORLD_WIDTH * WORLD_WIDTH;
  const snakeStartIndex = rnd(WORLD_SIZE);
  const world = World.new(WORLD_WIDTH, snakeStartIndex);
  const worldWidth = world.width();

  const gameControlBtn = <HTMLButtonElement> document.getElementById("game-control-btn");
  const canvas = <HTMLCanvasElement> document.getElementById("snake-canvas");
  const ctx = canvas.getContext("2d");

  canvas.width = worldWidth * CELL_SIZE;
  canvas.height = worldWidth * CELL_SIZE;

  gameControlBtn.addEventListener("click", _ => {
    if (world.game_status() === undefined) {
      world.start_game();
      play();
      gameControlBtn.textContent = "Reload";
    } else {
      location.reload();
    }
  });

  document.addEventListener("keydown", (event) => {
    switch (event.code) {
      case "KeyW":
      case "ArrowUp": world.set_snake_direction(Direction.Up); break;
      case "KeyS":
      case "ArrowDown": world.set_snake_direction(Direction.Down); break;
      case "KeyA":
      case "ArrowLeft": world.set_snake_direction(Direction.Left); break;
      case "KeyD":
      case "ArrowRight": world.set_snake_direction(Direction.Right); break;
    }
  })

  function drawWorld() {
    ctx.beginPath();

    for (let x = 0; x < worldWidth + 1; x++) {
      ctx.moveTo(CELL_SIZE * x, 0);
      ctx.lineTo(CELL_SIZE * x, CELL_SIZE * worldWidth);
    }

    for (let y = 0; y < worldWidth + 1; y++) {
      ctx.moveTo(0, CELL_SIZE * y);
      ctx.lineTo(CELL_SIZE * worldWidth, CELL_SIZE * y);
    }

    ctx.stroke();
  }

  function drawReward() {
    const rewardIdx = world.reward_cell();
    const col = rewardIdx % worldWidth;
    const row = Math.floor(rewardIdx / worldWidth);

    ctx.fillStyle = '#ee0000';
    ctx.beginPath();
    ctx.fillRect(col * CELL_SIZE, row * CELL_SIZE, CELL_SIZE, CELL_SIZE);
    ctx.stroke();
  }

  function drawSnake() {
    const snakeCells = new Uint32Array(
      wasm.memory.buffer,
      world.snake_cells(),
      world.snake_length()
    );

    const filledCells: number[] = [];

    snakeCells.forEach((cellIdx, i) => {
      const col = cellIdx % worldWidth;
      const row = Math.floor(cellIdx / worldWidth);

      if (!filledCells.includes(cellIdx)) {
        ctx.fillStyle = i === 0 ? '#7878db' : '#000000';
        ctx.beginPath();
        ctx.fillRect(col * CELL_SIZE, row * CELL_SIZE, CELL_SIZE, CELL_SIZE);

        filledCells.push(cellIdx);
      }
    })

    ctx.stroke();
  }

  function paint() {
    drawWorld();
    drawSnake();
    drawReward();
  }

  function play() {
    setTimeout(() => {
      ctx.clearRect(0, 0, canvas.width, canvas.height);
      world.step();
      paint();
      if (world.reward_cell() < WORLD_SIZE) {
        requestAnimationFrame(play);
      } else {
        alert("You won");
      }
    }, 1000 / FPS)
  }

  paint();

})