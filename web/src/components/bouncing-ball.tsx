import { useEffect, useRef } from "react";

type BouncingBallProps = {
  className?: string;
};

export function BouncingBall({ className }: BouncingBallProps) {
  const canvasRef = useRef<HTMLCanvasElement>(null);

  useEffect(() => {
    const canvas = canvasRef.current;
    if (!canvas) return;

    const ctx = canvas.getContext("2d");
    if (!ctx) return;

    let x = 50;
    let y = 50;
    let dx = 2;
    let dy = 2;
    const ballRadius = 20;

    function drawBall() {
      if (!ctx) return;
      ctx.beginPath();
      ctx.arc(x, y, ballRadius, 0, Math.PI * 2);
      ctx.fillStyle = `hsl(${getComputedStyle(document.body).getPropertyValue("--primary")})`;
      ctx.fill();
      ctx.closePath();
    }

    function animate() {
      if (!ctx || !canvas) return;

      ctx.clearRect(0, 0, canvas.width, canvas.height);
      drawBall();

      if (x + dx > canvas.width - ballRadius || x + dx < ballRadius) {
        dx = -dx;
      }
      if (y + dy > canvas.height - ballRadius || y + dy < ballRadius) {
        dy = -dy;
      }

      x += dx;
      y += dy;

      requestAnimationFrame(animate);
    }

    function resizeCanvas() {
      if (!canvas) return;
      canvas.width = window.innerWidth;
      canvas.height = window.innerHeight - 65; // Subtracting header height
    }

    resizeCanvas();
    window.addEventListener("resize", resizeCanvas);

    animate();

    return () => {
      window.removeEventListener("resize", resizeCanvas);
    };
  }, []);

  return <canvas ref={canvasRef} className={className} />;
}
