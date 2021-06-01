export class Terminal {
    constructor(inputEl, outputEl) {
        this.inputEl = inputEl;
        this.outputEl = outputEl;

        this.state = {
            id: 'idle',
        };

        this.history = [];

        this.inputEl.onkeyup = (event) => {
            switch (event.keyCode) {
                case 13:
                    this._handleEnter(event);
                    break;

                case 38:
                    this._handleArrowUp(event);
                    break;

                case 40:
                    this._handleArrowDown(event);
                    break;

                default:
                    if (this.state.id == 'browsing-history') {
                        this.state = {
                            id: 'idle',
                        };
                    }
            }
        };

        this.onInputHandler = (_line) => void 0;
    }

    onInput(fn) {
        this.onInputHandler = fn;
    }

    println(msg) {
        if (this.outputEl.value) {
            this.outputEl.value += '\n';
        }

        this.outputEl.value += msg;
        this.outputEl.scrollTop = this.outputEl.scrollHeight;
    }

    scrollToTop() {
        this.inputEl.focus();
        this.outputEl.scrollTop = 0;
    }

    _handleEnter(event) {
        event.preventDefault();

        const input = this.inputEl.value.trim();

        if (input.length > 0) {
            this.history.push(input);
            this.onInputHandler(input);
        }

        this.inputEl.value = '';

        this.state = {
            id: 'idle',
        };
    }

    _handleArrowUp(event) {
        event.preventDefault();

        switch (this.state.id) {
            case 'idle':
                if (this.history.length == 0) {
                    return;
                }

                if (this.inputEl.value.length > 0) {
                    return;
                }

                this.state = {
                    id: 'browsing-history',
                    historyIdx: 1,
                };

                this.inputEl.value = this.history[this.history.length - 1];
                break;

            case 'browsing-history':
                if (this.state.historyIdx < this.history.length) {
                    this.state.historyIdx += 1;

                    this.inputEl.value = this.history[
                        this.history.length - this.state.historyIdx
                    ];
                } else {
                    this.inputEl.value = '';
                }

                break;
        }
    }

    _handleArrowDown(event) {
        event.preventDefault();

        if (this.state.id == 'browsing-history') {
            if (this.state.historyIdx > 1) {
                this.state.historyIdx -= 1;

                this.inputEl.value = this.history[
                    this.history.length - this.state.historyIdx
                ];
            } else {
                this.inputEl.value = '';
            }
        }
    }
}
