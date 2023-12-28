const BOARD_SIZE = 9;
const PIECE_BUFFER = 0.1;

function getLine(x1, y1, x2, y2, strokeWidth) {
  const line = document.createElementNS("http://www.w3.org/2000/svg", "line");
  line.setAttribute("x1", x1);
  line.setAttribute("y1", y1);
  line.setAttribute("x2", x2);
  line.setAttribute("y2", y2);
  line.setAttribute("stroke", "black");
  line.setAttribute("stroke-width", strokeWidth);
  return line;
}

function drawBoard() {
  const gameSvg = document.getElementById("svg-game");

  // grid lines
  for (let i = 0; i < BOARD_SIZE + 1; i++) {
    const strokeWidth = i % 3 === 0 ? 0.05 : 0.01;
    const horizontalLine = getLine(i, 0, i, BOARD_SIZE, strokeWidth);
    gameSvg.appendChild(horizontalLine);
    const verticalLine = getLine(0, i, BOARD_SIZE, i, strokeWidth);
    gameSvg.appendChild(verticalLine);
  }

  // grid squares
  for (let x = 0; x < BOARD_SIZE; x++) {
    for (let y = 0; y < BOARD_SIZE; y++) {
      const square = document.createElementNS(
        "http://www.w3.org/2000/svg",
        "rect"
      );
      square.id = `square-${x}-${y}`;
      square.setAttribute("x", x + PIECE_BUFFER / 2);
      square.setAttribute("y", y + PIECE_BUFFER / 2);
      square.setAttribute("width", 1 - PIECE_BUFFER);
      square.setAttribute("height", 1 - PIECE_BUFFER);
      square.setAttribute("data-x", x);
      square.setAttribute("data-y", y);
      square.setAttribute("fill", "black");
      square.setAttribute("opacity", 0);
      square.addEventListener("click", (event) => {
        requestAction(
          event.target.getAttribute("data-x"),
          event.target.getAttribute("data-y")
        );
      });
      gameSvg.appendChild(square);
    }
  }
}

function drawNought(x, y, size) {
  const circle = document.createElementNS(
    "http://www.w3.org/2000/svg",
    "circle"
  );
  circle.id = `piece-${x}-${y}-${size}`;
  circle.setAttribute("cx", x * size + size / 2);
  circle.setAttribute("cy", y * size + size / 2);
  circle.setAttribute("r", (size - 2 * PIECE_BUFFER * size) / 2);
  circle.setAttribute("fill", "none");
  circle.setAttribute("stroke", "black");
  circle.setAttribute("stroke-width", 0.03 * size);
  document.getElementById("svg-game").appendChild(circle);
}

function drawCross(x, y, size) {
  const line1 = getLine(
    (x + PIECE_BUFFER) * size,
    (y + PIECE_BUFFER) * size,
    (x + (1 - PIECE_BUFFER)) * size,
    (y + (1 - PIECE_BUFFER)) * size,
    0.03 * size
  );
  const line2 = getLine(
    (x + (1 - PIECE_BUFFER)) * size,
    (y + PIECE_BUFFER) * size,
    (x + PIECE_BUFFER) * size,
    (y + (1 - PIECE_BUFFER)) * size,
    0.03 * size
  );
  const lineGroup = document.createElementNS("http://www.w3.org/2000/svg", "g");
  lineGroup.id = `piece-${x}-${y}-${size}`;
  lineGroup.appendChild(line1);
  lineGroup.appendChild(line2);
  document.getElementById("svg-game").appendChild(lineGroup);
}

function drawPiece(piece, x, y, size) {
  switch (piece) {
    case "Nought":
      drawNought(x, y, size);
      break;
    case "Cross":
      drawCross(x, y, size);
      break;
    default:
      const existingPiece = document.getElementById(`piece-${x}-${y}-${size}`);
      if (existingPiece) {
        existingPiece.remove();
      }
  }
}

function drawPieces(gameState) {
  for (let meta_x = 0; meta_x < 3; meta_x++) {
    for (let meta_y = 0; meta_y < 3; meta_y++) {
      const metaPiece = gameState.meta_pieces[meta_x][meta_y];
      drawPiece(metaPiece, meta_x, meta_y, 3);
      for (let x = 0; x < 3; x++) {
        for (let y = 0; y < 3; y++) {
          const piece = gameState.pieces[meta_x][meta_y][x][y];
          drawPiece(piece, meta_x * 3 + x, meta_y * 3 + y, 1);
          const square = document.getElementById(
            `square-${meta_x * 3 + x}-${meta_y * 3 + y}`
          );
          if (
            !piece &&
            !metaPiece &&
            (!gameState.meta_coords_restriction ||
              (gameState.meta_coords_restriction.x == meta_x &&
                gameState.meta_coords_restriction.y == meta_y))
          ) {
            square.setAttribute("fill", "green");
            square.setAttribute("opacity", 0.5);
            square.onmouseover = function (event) {
              event.target.setAttribute("opacity", 1);
            };
            square.onmouseout = function (event) {
              event.target.setAttribute("opacity", 0.5);
            };
          } else {
            square.onmouseover = null;
            square.onmouseout = null;
            square.setAttribute("opacity", 0);
          }
        }
      }
    }
  }
}

function requestAction(x, y) {
  let xNum = Number(x);
  let yNum = Number(y);
  console.log("Sending", JSON.stringify({ xNum, yNum }));
  socket.send(JSON.stringify({ x: xNum, y: yNum }));
}

const socket = new WebSocket("ws://127.0.0.1:8080");

socket.addEventListener("open", (event) => {
  console.log("Socket connection opened", event);
});

socket.addEventListener("message", (event) => {
  console.log("Message from server ", event.data);
  const gameState = JSON.parse(event.data);
  if ("game_over" in gameState) {
    drawPieces(gameState);
    if (gameState.game_over) {
      alert("Game Over!");
    }
  }
});

drawBoard();
