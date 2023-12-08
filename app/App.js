import React from 'react';
import ReactDOM from 'react-dom';
import { run_bf } from '../pkg'
import Button from '@mui/material/Button';
import TextField from '@mui/material/TextField';
import Box from "@mui/material/Box";
import Typography from "@mui/material/Typography";
import Stack from '@mui/material/Stack';
import Container from '@mui/material/Container';
import { styled } from '@mui/material/styles';
import Paper from '@mui/material/Paper';

const Item = styled(Paper)(({ theme }) => ({
    backgroundColor: theme.palette.mode === 'dark' ? '#1A2027' : '#fff',
    ...theme.typography.body2,
    padding: theme.spacing(1),
    textAlign: 'center',
    color: theme.palette.text.secondary,
}));

const root = document.getElementById('root');

let predef = "++++++++++[>+++++++>++++++++++>+++>+<<<<-]>++.>+.+++++++..+++.>++.<<+++++++++++++++.>.+++.------.--------.>+.>."
let defconfig = { cells: 128, fuel: 4096 };

const App = () => {
    const [prog, setProg] = React.useState(predef);
    const [input, setInput] = React.useState('');
    const [config, setConfig] = React.useState(defconfig)
    const [output, setOutput] = React.useState(run_bf(predef, '', defconfig.cells, defconfig.fuel));
    return (
        <Container component="main" maxWidth="xs">
            <Box sx={{ width: '100%' }}>
                <Stack spacing={2}>
                    <Item><Typography variant="h2">
                        🫠
                    </Typography></Item>
                    <Item><TextField
                        fullWidth
                        multiline
                        id="prog"
                        label="bf program"
                        name="prog"
                        variant="outlined"
                        defaultValue={prog}
                        onChange={(e) =>
                            setProg(e.target.value)
                        }
                        autoFocus /></Item>
                    <Item><TextField
                        fullWidth
                        id="input"
                        label="input buffer"
                        name="input"
                        variant="outlined"
                        defaultValue={input}
                        onChange={(e) =>
                            setInput(e.target.value)
                        }
                        autoFocus /></Item>
                    <Item><Stack direction="row" spacing={1}>
                        <Item><TextField
                            fullWidth
                            id="cells"
                            label="cell size"
                            name="cells"
                            variant="outlined"
                            defaultValue={defconfig.cells}
                            onChange={(e) => {
                                if (e.target.value < 1) { e.target.value = 1 }
                                setConfig({ cells: Number(e.target.value), fuel: config.fuel })
                            }}
                            type='number'
                            autoFocus /></Item>
                        <Item><TextField
                            fullWidth
                            id="fuel"
                            label="max steps"
                            name="fuel"
                            variant="outlined"
                            defaultValue={defconfig.fuel}
                            onChange={(e) => {
                                if (e.target.value < 1) { e.target.value = 1 }
                                setConfig({ cells: config.cells, fuel: Number(e.target.value) })
                            }}
                            type='number'
                            autoFocus /></Item>
                    </Stack></Item>
                    <Item><Button
                        type="submit"
                        fullWidth
                        variant="contained"
                        onClick={() => setOutput(run_bf(prog, input, config.cells, config.fuel))}
                    >
                        Run
                    </Button></Item>
                    <Item><Typography>
                        Output: {output}
                    </Typography></Item>
                </Stack>
            </Box>
        </Container>
    );
}

ReactDOM.render(<App />, root);
