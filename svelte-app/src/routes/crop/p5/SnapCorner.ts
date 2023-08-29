/**
 * p5 class: SnapCorner
 */

import type p5 from 'p5';
import SnapMember from './SnapMember';
import type SnapEdge from './SnapEdge';
import type SnapCenter from './SnapCenter';

/**
 * This class represents the corners of the crop area. It is used to resize the crop area.
 */
export default class SnapCorner extends SnapMember {

    constructor(p5: p5, id: string, vec: { x: number, y: number }) {

        super(p5, id, vec.x, vec.y);

    }

    reposition(edge: SnapEdge | SnapCenter, corner: SnapCorner) {
        this.pos.x = corner.pos.x + 2 * (edge.pos.x - corner.pos.x);
        this.pos.y = corner.pos.y + 2 * (edge.pos.y - corner.pos.y);
    }


    public override draw(p5: p5): void {

        p5.stroke(51);

        if (this.isDragged) {
            p5.fill(51);
        } else {
            p5.fill(250);
        }

        p5.ellipse(this.pos.x, this.pos.y, 20, 20);

    }

    toVector(): { x: number, y: number } {
        return { x: this.pos.x, y: this.pos.y };
    }

    protected override mousePressIsWithinContainer(p5: p5): boolean {
        const dx = p5.mouseX - this.pos.x;
        const dy = p5.mouseY - this.pos.y;
        const dist = Math.sqrt(dx * dx + dy * dy);

        return dist < 50;
    }

    /**
     * p5 method: resize
     * @param p5 - p5 instance
     */
    resize(p5: p5, oldWidth: number, oldHeight: number) {

        this.pos.x = (this.pos.x / oldWidth) * p5.width;
        this.pos.y = (this.pos.y / oldHeight) * p5.height;

    }


}