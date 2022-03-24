<script>
  import { Universe, Cell } from "wasm-game-of-life";
  import { onMount } from "svelte";

  const CELL_SIZE = 16; // px
  const GRID_COLOR = "#CCCCCC";
  const DEAD_COLOR = "#FFFFFF";
  const ALIVE_COLOR = "#000000";
 
  const universe = Universe.new();
  const w = universe.width();
  const h = universe.height();
  const canvasWidth = (CELL_SIZE + 1) * universe.width() + 1;
  const canvasHeight = (CELL_SIZE + 1) * universe.height() + 1;

  let canvas;

  const drawGrid = ( context ) => {
     // draw grid
    context.beginPath();
    context.strokeStyle = GRID_COLOR;
    // Vertical lines.
    for (let i = 0; i <= w; i++) {
    context.moveTo(i * (CELL_SIZE + 1) + 1, 0);
    context.lineTo(i * (CELL_SIZE + 1) + 1, (CELL_SIZE + 1) * h + 1);
    }
    // Horizontal lines.
    for (let j = 0; j <= h; j++) {
    context.moveTo(0,                           j * (CELL_SIZE + 1) + 1);
    context.lineTo((CELL_SIZE + 1) * w + 1, j * (CELL_SIZE + 1) + 1);
    }
    context.stroke();
  };

  const getIndex = (row, column) => {
    return row * w + column;
  };

  const drawCells = (ctx) => {
    const cells = universe.cells_copy();

    ctx.beginPath();

    for (let row = 0; row < h; row++) {
        for (let col = 0; col < w; col++) {
          const idx = getIndex(row, col);

          ctx.fillStyle = cells[idx] === Cell.Alive
            ? ALIVE_COLOR
            : DEAD_COLOR;

          ctx.fillRect(
            col * (CELL_SIZE + 1) + 1,
            row * (CELL_SIZE + 1) + 1,
            CELL_SIZE,
            CELL_SIZE
          );
        }
    }

    ctx.stroke(); 
  };

  onMount(() => {
    const ctx = canvas.getContext('2d');
    let frame = requestAnimationFrame(loop);
    function loop() {
        universe.tick();
        drawGrid(ctx);
        drawCells(ctx);
        frame = requestAnimationFrame(loop)
    }
    return () => {
        cancelAnimationFrame(frame);
    };
  });
</script>

<canvas
    bind:this={canvas}
    width={canvasWidth}
    height={canvasHeight}
></canvas>
