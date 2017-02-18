use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;

use Sample;
use Source;

/// Filter that allows another thread to pause the stream.
#[derive(Clone, Debug)]
pub struct SignalWhenOver<I>
    where I: Source,
          I::Item: Sample
{
    input: I,
    // The finished value which will be read by another thread.
    remote_finished: Arc<AtomicBool>,
}

impl<I> SignalWhenOver<I>
    where I: Source,
          I::Item: Sample
{
    pub fn new(source: I, remote_finished: Arc<AtomicBool>) -> SignalWhenOver<I> {
        // TODO: handle the fact that the samples rate can change
        SignalWhenOver {
            input: source,
            remote_finished: remote_finished,
        }
    }
}

impl<I> Iterator for SignalWhenOver<I>
    where I: Source,
          I::Item: Sample
{
    type Item = I::Item;

    #[inline]
    fn next(&mut self) -> Option<I::Item> {
        let next = self.input.next();
        if next.is_none() {
            self.remote_finished.store(true, Ordering::Relaxed);
        }
        next
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.input.size_hint()
    }
}

impl<I> Source for SignalWhenOver<I>
    where I: Source,
          I::Item: Sample
{
    #[inline]
    fn get_current_frame_len(&self) -> Option<usize> {
        self.input.get_current_frame_len()
    }

    #[inline]
    fn get_channels(&self) -> u16 {
        self.input.get_channels()
    }

    #[inline]
    fn get_samples_rate(&self) -> u32 {
        self.input.get_samples_rate()
    }

    #[inline]
    fn get_total_duration(&self) -> Option<Duration> {
        self.input.get_total_duration()
    }
}
