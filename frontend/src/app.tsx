import {
    ChangeEvent,
    Dispatch,
    FormEvent,
    SetStateAction,
    useEffect,
    useState,
} from "react";
import { solve24 } from "../pkg/twentyfour_wasm";
import init from "../pkg/twentyfour_wasm_bg.wasm?init";
import "./index.scss";

interface Solution {
    solution: string;
    explanation: string[];
}

const MIN = 1;
const MAX = 99;

const isValid = (n: number) => n >= MIN && n <= MAX;

const App = () => {
    const [n1, setN1] = useState(1);
    const [n2, setN2] = useState(2);
    const [n3, setN3] = useState(3);
    const [n4, setN4] = useState(4);
    const [sols, setSols] = useState<Solution[]>([]);
    const [touched, setTouched] = useState(false);

    // avoid repeat calls
    const [inputsChangedSinceBtnClick, setInputsChangedSinceClick] =
        useState(true);

    const inputsValid = [n1, n2, n3, n4].every((x) => isValid(x));

    useEffect(() => {
        init({});
    }, []);

    const handleNChange =
        (setter: Dispatch<SetStateAction<number>>) =>
        (e: ChangeEvent<HTMLInputElement>) => {
            if (e.target.validity.valid) {
                setInputsChangedSinceClick(true);
                setter(e.target.value as unknown as number);
            }
        };

    const handleBtnClick = () => {
        if (inputsChangedSinceBtnClick) {
            const sols: Solution[] = JSON.parse(solve24(n1, n2, n3, n4));
            setTouched(true);
            setSols(sols);
        }
        setInputsChangedSinceClick(false);
    };

    return (
        <main>
            <div>
                <p>
                    Enter four numbers below.
                    <br />
                    Click &ldquo;Solve&rdquo; to see every solution that equals
                    24.
                </p>

                <div id="inputs">
                    <Input value={n1} onChange={handleNChange(setN1)} />
                    <Input value={n2} onChange={handleNChange(setN2)} />
                    <Input value={n3} onChange={handleNChange(setN3)} />
                    <Input value={n4} onChange={handleNChange(setN4)} />
                    <button
                        onClick={handleBtnClick}
                        disabled={!inputsValid}
                        className={!inputsValid ? "disabled" : ""}
                    >
                        Solve
                    </button>
                </div>
            </div>
            <div>
                <p
                    id="num-sols"
                    style={{
                        visibility:
                            touched && inputsValid && sols.length
                                ? "visible"
                                : "hidden",
                    }}
                >
                    {sols.length} solution{sols.length !== 1 && "s"}
                </p>
                <table>
                    <thead>
                        <tr>
                            <th className="num-col">No.</th>
                            <th className="sol-col">Solution</th>
                            <th className="exp-col">Explanation</th>
                        </tr>
                    </thead>
                    <tbody>
                        {sols.length
                            ? sols.map((sol, i) => (
                                  <tr key={i}>
                                      <td className="num-col">{i + 1}</td>
                                      <td className="sol-col">
                                          {sol.solution} = 24
                                      </td>
                                      <td className="exp-col">
                                          {sol.explanation.map((x, j) => (
                                              <div
                                                  key={j}
                                                  dangerouslySetInnerHTML={{
                                                      __html: x,
                                                  }}
                                              />
                                          ))}
                                      </td>
                                  </tr>
                              ))
                            : touched && (
                                  <tr>
                                      <td colSpan={3} id="no-sol-cell">
                                          No solutions
                                      </td>
                                  </tr>
                              )}
                    </tbody>
                </table>
            </div>
        </main>
    );
};

export default App;

const Input = ({
    value,
    onChange,
}: {
    value: number;
    onChange: (e: ChangeEvent<HTMLInputElement>) => void;
}) => {
    const [valid, setValid] = useState(true);

    const handleInput = (e: FormEvent<HTMLInputElement>) => {
        console.log(e.currentTarget.value);
        setValid(isValid(e.currentTarget.value as unknown as number));
    };

    return (
        <input
            className={valid ? "" : "invalid"}
            value={value}
            type="number"
            pattern="[0-9]*"
            maxLength={2}
            onInput={handleInput}
            onChange={onChange}
        />
    );
};
