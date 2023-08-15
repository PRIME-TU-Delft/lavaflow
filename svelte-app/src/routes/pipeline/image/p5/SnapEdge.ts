/**
 * p5 class: SnapEdge
 */

import type p5 from 'p5';
import SnapMember from './SnapMember';
import type SnapCorner from './SnapCorner';

export default class SnapEdge extends SnapMember {

    constructor(p5: p5, id: string, a: SnapCorner, b: SnapCorner) {

        super(p5, id, 0, 0);

        this.reposition(a, b);

    }

    reposition(a: SnapCorner, b: SnapCorner) {
        this.pos.x = (a.pos.x + b.pos.x) / 2;
        this.pos.y = (a.pos.y + b.pos.y) / 2;
    }


    public override draw(p5: p5): void {

        // Declare variable to keep track of rotation angle
        let angle = 0;

        p5.stroke(51);

        // If this edge drags along two corners, draw a line between those corners
        if (this.dragAlongMembers.length == 2) {
            const c1 = this.dragAlongMembers[0];
            const c2 = this.dragAlongMembers[1];

            p5.line(c1.pos.x, c1.pos.y, c2.pos.x, c2.pos.y);

            // Rotate such that the edge perfectly lines up with the incline of the edge
            const dx = c2.pos.x - c1.pos.x;
            const dy = c2.pos.y - c1.pos.y;

            angle = Math.atan(dy / dx);
        }

        if (this.isDragged) {
            p5.fill(51);
        } else {
            p5.fill(250);
        }

        p5.translate(this.pos.x, this.pos.y);
        p5.rotate(angle);

        p5.rectMode(p5.CENTER);
        p5.rect(0, 0, 50, 10);

        p5.rotate(-angle);
        p5.translate(-this.pos.x, -this.pos.y);

    }


    protected override mousePressIsWithinContainer(p5: p5): boolean {
        const dx = p5.mouseX - this.pos.x;
        const dy = p5.mouseY - this.pos.y;
        const dist = Math.sqrt(dx * dx + dy * dy);

        return dist < 50;
    }


}