export class ViewportComponent {
    constructor(el) {
        this.el = el;
    }

    render({foods, animals}) {
        const frame = this._prepareFrame();

        for (const food of foods) {
            this._renderFood(frame, food);
        }

        for (const animal of animals) {
            this._renderAnimal(frame, animal);
        }
    }

    _prepareFrame() {
        const ctxt = this.el.getContext('2d');

        const rect = {
            width: window.innerWidth - 500, // TODO hardcoded
            height: window.innerHeight - 50, // TODO hardcoded
        };

        const dpr = window.devicePixelRatio || 1;
        const size = Math.min(rect.width, rect.height);

        this.el.width = size * dpr;
        this.el.height = size * dpr;
        this.el.style.width = size + 'px';
        this.el.style.height = size + 'px';

        ctxt.scale(dpr, dpr);

        return {ctxt, size};
    }

    _renderFood({ctxt, size}, {position}) {
        const x = position.x * size;
        const y = position.y * size;

        ctxt.beginPath();
        ctxt.arc(x, y, 3, 0, 2.0 * Math.PI);
        ctxt.fillStyle = '#00ff66';
        ctxt.fill();
    }

    _renderAnimal({ctxt, size}, {position, rotation, eyeCells}) {
        const FOV = Math.PI; // TODO

        const x = position.x * size;
        const y = position.y * size;
        const r = Math.PI - rotation;

        ctxt.beginPath();
        ctxt.moveTo(x, y);
        ctxt.lineTo(x + Math.sin(r - 0.4) * 12.0, y + Math.cos(r - 0.4) * 12.0);
        ctxt.lineTo(x + Math.sin(r + 0.4) * 12.0, y + Math.cos(r + 0.4) * 12.0);
        ctxt.lineTo(x, y);
        ctxt.fillStyle = 'white';
        ctxt.fill();

        const fovPerCell = FOV / eyeCells.length;
        const fovStart = -r + eyeCells.length * fovPerCell;

        for (let eyeCellId = 0; eyeCellId < eyeCells.length; eyeCellId += 1) {
            const startAngle = (fovStart + eyeCellId * fovPerCell - fovPerCell / 2.0) % (2.0 * Math.PI);
            const angleTo = (startAngle + fovPerCell) % (2.0 * Math.PI);
            const energy = eyeCells[eyeCellId];

            ctxt.beginPath();
            ctxt.arc(x, y, 15, startAngle, angleTo);
            ctxt.strokeStyle = `rgba(000, 255, 102, ${energy})`;
            ctxt.stroke();
        }
    }
}
