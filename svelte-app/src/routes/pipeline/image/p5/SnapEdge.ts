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


}