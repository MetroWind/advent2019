use std::vec::Vec;

use crate::intcode::intcode;

fn permutations_inner<T, F, R>(xs: &mut Vec<&T>, end: usize, do_what: &mut F)
where F: FnMut(&Vec<&T>) -> R
{
    if end == 1
    {
        do_what(xs);
        return;
    }

    for i in 0..end
    {
        permutations_inner(xs, end - 1, do_what);

        if end % 2 == 1
        {
            // if size is odd, swap first and last element
            xs.swap(0, end - 1);
        }
        else
        {
            // If size is even, swap ith and last element
            xs.swap(i, end - 1);
        }
    }
}

fn permutations<T, F, R>(xs: &[T], mut do_what: F) where F: FnMut(&Vec<&T>) -> R
{
    let mut refer: Vec<&T> = xs.iter().map(|x| x).collect();
    permutations_inner(&mut refer, xs.len(), &mut do_what);
}

struct Amplifier
{
    pub phase: i32,
    pub input: i32,
    pub output: i32,
    pub core: intcode::IntCodeComputer,
    pub stopped: bool,

    code: Vec<i32>,
}

impl Amplifier
{
    pub fn new(code: &Vec<i32>) -> Amplifier
    {
        Amplifier
        {
            phase: 0,
            input: 0,
            output: 0,
            core: intcode::IntCodeComputer::new(),
            stopped: false,
            code: code.clone(),
        }
    }

    pub fn runOnce(&mut self)
    {
        self.core.eval(&self.code, Some(&vec![self.phase, self.input]));
        self.output = self.core.output[0];
        self.core.reset();
    }

    pub fn run(&mut self)
    {
        if let Some(output) = self.core.pipe(self.input)
        {
            self.output = output;
        }
        else
        {
            self.stopped = true;
        }
    }
}

fn testAmpsWithPhases(amps: &mut [Amplifier], first_input: i32,
                      phases: &Vec<i32>) -> i32
{
    if amps.len() == 0
    {
        return first_input;
    }

    let mut input: i32 = first_input;
    for i in 0..amps.len()
    {
        amps[i].input = input;
        amps[i].phase = phases[i];
        amps[i].runOnce();
        input = amps[i].output;
    }
    input
}

fn testAmps(amps: &mut [Amplifier], first_input: i32) -> i32
{
    let phases: Vec<i32> = (0..amps.len()).map(|i| i as i32).collect();
    let mut max_output = i32::min_value();

    permutations(
        &phases[..],
        |perm|
        {
            let this_phases: Vec<i32> = perm.iter().map(|&x| x.clone()).collect();
            let output = testAmpsWithPhases(amps, first_input, &this_phases);
            max_output = output.max(max_output);
        });

    max_output
}

fn feedbackWithPhases(amps: &mut [Amplifier], first_input: i32,
                      phases: &Vec<i32>) -> i32
{
    for i in 0..amps.len()
    {
        amps[i].stopped = false;
        amps[i].core.reset();
        amps[i].core.mem = amps[i].code.clone();
        amps[i].core.consumeSingleInput(phases[i]);
    }

    let mut input = first_input;
    // print!("{} ", input);
    let mut output = 0;
    loop
    {
        for i in 0..amps.len()
        {
            amps[i].input = input;
            amps[i].run();
            input = amps[i].output;
            // print!("--> {} ", input);
            if amps[i].stopped
            {
                return output;
            }
        }
        output = amps.last().unwrap().output;
        // println!();
    }
}

fn feedback(amps: &mut [Amplifier], first_input: i32) -> i32
{
    let phases: Vec<i32> = (5..=9).map(|i| i as i32).collect();
    let mut max_output = i32::min_value();

    permutations(
        &phases[..],
        |perm|
        {
            let this_phases: Vec<i32> = perm.iter().map(|&x| x.clone()).collect();
            let output = feedbackWithPhases(amps, first_input, &this_phases);
            max_output = output.max(max_output);
        });

    max_output
}

pub fn part1(input: &str) -> i32
{
    let code = intcode::parse(input);
    let mut amps: Vec<Amplifier> = (0..5).map(|_| Amplifier::new(&code)).collect();
    testAmps(&mut amps[..], 0)
}

pub fn part2(input: &str) -> i32
{
    let code = intcode::parse(input);
    let mut amps: Vec<Amplifier> = (0..5).map(|_| Amplifier::new(&code)).collect();
    feedback(&mut amps[..], 0)
}

#[test]
fn testPart1()
{
    {
        let code: Vec<i32> = vec![3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0];
        let mut amps: Vec<Amplifier> = (0..5).map(|_| Amplifier::new(&code)).collect();
        let phases: Vec<i32> = vec![4,3,2,1,0];
        let output = testAmpsWithPhases(&mut amps[..], 0, &phases);
        assert_eq!(output, 43210);
    }

    {
        let code: Vec<i32> = vec![3,23,3,24,1002,24,10,24,1002,23,-1,23,
                                  101,5,23,23,1,24,23,23,4,23,99,0,0];
        let mut amps: Vec<Amplifier> = (0..5).map(|_| Amplifier::new(&code)).collect();
        let phases: Vec<i32> = vec![0,1,2,3,4];
        let output = testAmpsWithPhases(&mut amps[..], 0, &phases);
        assert_eq!(output, 54321);
    }

    {
        let code: Vec<i32> = vec![3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0];
        let mut amps: Vec<Amplifier> = (0..5).map(|_| Amplifier::new(&code)).collect();
        let output = testAmps(&mut amps[..], 0);
        assert_eq!(output, 43210);
    }

    {
        let code: Vec<i32> = vec![3,23,3,24,1002,24,10,24,1002,23,-1,23,
                                  101,5,23,23,1,24,23,23,4,23,99,0,0];
        let mut amps: Vec<Amplifier> = (0..5).map(|_| Amplifier::new(&code)).collect();
        let output = testAmps(&mut amps[..], 0);
        assert_eq!(output, 54321);
    }

    {
        let code: Vec<i32> = vec![3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,
                                  1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0];
        let mut amps: Vec<Amplifier> = (0..5).map(|_| Amplifier::new(&code)).collect();
        let output = testAmps(&mut amps[..], 0);
        assert_eq!(output, 65210);
    }
}

#[test]
fn testPart2()
{
    {
        let code: Vec<i32> = vec![3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,
                                  27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5];

        let mut amps: Vec<Amplifier> = (0..5).map(|_| Amplifier::new(&code)).collect();
        let phases: Vec<i32> = vec![9,8,7,6,5];
        let output = feedbackWithPhases(&mut amps[..], 0, &phases);
        assert_eq!(output, 139629729);
    }

    {
        let code: Vec<i32> = vec![
            3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,
            -5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,
            53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10];

        let mut amps: Vec<Amplifier> = (0..5).map(|_| Amplifier::new(&code)).collect();
        let phases: Vec<i32> = vec![9,7,8,5,6];
        let output = feedbackWithPhases(&mut amps[..], 0, &phases);
        assert_eq!(output, 18216);
    }

    {
        let code: Vec<i32> = vec![3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,
                                  27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5];

        let mut amps: Vec<Amplifier> = (0..5).map(|_| Amplifier::new(&code)).collect();
        let output = feedback(&mut amps[..], 0);
        assert_eq!(output, 139629729);
    }

    {
        let code: Vec<i32> = vec![
            3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,
            -5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,
            53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10];

        let mut amps: Vec<Amplifier> = (0..5).map(|_| Amplifier::new(&code)).collect();
        let output = feedback(&mut amps[..], 0);
        assert_eq!(output, 18216);
    }
}
