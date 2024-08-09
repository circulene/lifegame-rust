import { CanvasHTMLAttributes } from "react"
import useCanvas from "./CanvasHook"

export type DrawFunction = (context: CanvasRenderingContext2D, frameCount: number) => void

interface CanvasProps extends CanvasHTMLAttributes<HTMLCanvasElement> {
    draw: DrawFunction
}

function Canvas(props: CanvasProps) {
    const { draw, ...canvasProps } = props
    const canvasRef = useCanvas(draw)

    return <canvas ref={canvasRef} {...canvasProps} />
}

export default Canvas