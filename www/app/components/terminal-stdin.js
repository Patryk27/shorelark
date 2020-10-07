export class TerminalStdinComponent {
    constructor(el) {
        this.onInputHandler = (_line) => void 0;

        this.el = el;
        this.el.focus();

        this.el.onkeyup = (event) => {
            if (event.keyCode === 13) {
                event.preventDefault();

                const cmd = this.el.value.trim();

                if (cmd.length > 0) {
                    this.onInputHandler(cmd);
                }

                this.el.value = '';
            }
        };
    }

    onInput(fn) {
        this.onInputHandler = fn;
    }
}
