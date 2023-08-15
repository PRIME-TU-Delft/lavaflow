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

    repositionWithEdges(a: SnapEdge, b: SnapEdge) {
        this.pos.x = (a.pos.x + b.pos.x) / 2;
        this.pos.y = (a.pos.y + b.pos.y) / 2;
    }




}