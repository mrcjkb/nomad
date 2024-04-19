use core::ops::Range;

use nomad::test::{Generate, Generator, MeanLen, ReplacementCtx};
use nomad::{ByteOffset, NvimBuffer, Replacement, Shared};

#[nomad::test]
fn nomad_buffer_sync_fuzz_0(gen: &mut Generator) {
    buffer_sync(8, gen)
}

#[nomad::test]
fn nomad_buffer_sync_fuzz_1(gen: &mut Generator) {
    buffer_sync(32, gen)
}

#[nomad::test]
fn nomad_buffer_sync_fuzz_2(gen: &mut Generator) {
    buffer_sync(256, gen)
}

#[nomad::test]
fn nomad_buffer_sync_fuzz_3(gen: &mut Generator) {
    buffer_sync(1024, gen)
}

/// Tests that a `NvimBuffer` stays synced with a string after a series of
/// edits.
fn buffer_sync(num_edits: usize, gen: &mut Generator) {
    let mut buffer = NvimBuffer::create();

    let string = Shared::new(String::new());

    {
        let string = string.clone();

        buffer.on_edit(move |edit| {
            let range: Range<usize> = edit.start().into()..edit.end().into();
            string.with_mut(|s| s.replace_range(range, edit.replacement()));
        });
    }

    for _ in 0..num_edits {
        let replacement: Replacement<ByteOffset> = string.with(|s| {
            let ctx = ReplacementCtx::new(s.as_ref(), MeanLen(3), MeanLen(5));
            gen.generate(ctx)
        });

        buffer.edit(replacement);
    }

    string.with(|s| {
        assert_eq!(&buffer.get::<_, ByteOffset>(..).unwrap(), s);
    });
}
