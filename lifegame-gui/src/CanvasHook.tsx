import { useEffect, useRef } from "react";
import { DrawFunction } from "./Canvas";

function useCanvas(draw: DrawFunction) {
    const canvasRef = useRef(null)

    function getCanvas(): HTMLCanvasElement {
        const canvas: any = canvasRef.current
        if (canvas === null) {
            throw new Error("canvas is null")
        }
        return canvas
    }

    function getContext(): CanvasRenderingContext2D {
        const context: any = getCanvas().getContext('2d')
        if (context === null) {
            throw new Error("context is null")
        }
        return context
    }

    useEffect(() => {
        const context = getContext()
        let frameCount = 0
        let animationFrameId: number

        const render = () => {
            frameCount++
            draw(context, frameCount)
            animationFrameId = window.requestAnimationFrame(render)
        }
        render()

        return () => {
            window.cancelAnimationFrame(animationFrameId)
        }
    }, [draw])

    return canvasRef
}

export default useCanvas