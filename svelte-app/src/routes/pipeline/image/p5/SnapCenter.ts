/**
 * p5 class: SnapCenter
 */

import type p5 from 'p5';
import SnapMember from './SnapMember';
import type SnapCorner from './SnapCorner';
import type SnapEdge from './SnapEdge';

export default class SnapCenter extends SnapMember {

    constructor(p5: p5, id: string, a: SnapCorner, b: SnapCorner, c: SnapCorner, d: SnapCorner) {

        super(p5, id, 0, 0);
        this.reposition(a, b, c, d);

    }

    reposition(a: SnapCorner, b: SnapCorner, c: SnapCorner, d: SnapCorner) {
        this.pos.x = (a.pos.x + b.pos.x + c.pos.x + d.pos.x) / 4;
        this.pos.y = (a.pos.y + b.pos.y + c.pos.y + d.pos.y) / 4;
    }

    public override draw(p5: p5): void {

        p5.stroke(51, 100);

        if (this.isDragged) {
            p5.fill(51);
        } else {
            p5.fill(250, 100);
        }

        p5.ellipse(this.pos.x, this.pos.y, 20, 20);

    }

    protected override mousePressIsWithinContainer(p5: p5): boolean {
        const dx = p5.mouseX - this.pos.x;
        const dy = p5.mouseY - this.pos.y;
        const dist = Math.sqrt(dx * dx + dy * dy);

        return dist < 50;
    }




}