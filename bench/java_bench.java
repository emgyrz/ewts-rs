import java.io.IOException;
import java.nio.channels.FileChannel;
import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.Paths;
import java.time.Duration;
import java.time.Instant;

import io.bdrc.ewtsconverter.EwtsConverter;

public class java_bench {

  private final static long ITERATION_COUNT = 33;

  public static void main(final String[] args) 
  throws IOException {
    final Path path = Paths.get("./sample_ewts_text.txt");

    final double fileSize = FileChannel.open(path).size();
    final String ewtsText = String.join("\n", Files.readAllLines(path));

    final EwtsConverter converter = new EwtsConverter();

    int totalCharsCount = 0;

    final long start = System.currentTimeMillis();

    for (int i = 0; i < ITERATION_COUNT; i++) {
      totalCharsCount += converter.toUnicode(ewtsText).chars().count();
    }

    final long end = System.currentTimeMillis();
    final long timeElapsedMs = end - start;
    final double timeElapsedS = timeElapsedMs / 1000.0;

    final double speed = (fileSize * (double)ITERATION_COUNT / 1024.0) / timeElapsedS;

    System.out.println(String.format("java: speed - %.3f Kb/s; launches - %d; time - %d ms", speed, ITERATION_COUNT, timeElapsedMs));
  }


}

