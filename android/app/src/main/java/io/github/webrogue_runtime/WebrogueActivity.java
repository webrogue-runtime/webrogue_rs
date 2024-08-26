package io.github.webrogue_runtime;

import android.os.Bundle;
//import android.support.annotation.Keep;

import androidx.annotation.Keep;

import org.libsdl.app.SDLActivity;
import java.io.ByteArrayOutputStream;
import java.io.Console;
import java.io.IOException;
import java.io.InputStream;
import java.nio.charset.StandardCharsets;

public class WebrogueActivity extends SDLActivity {
    @Override
    protected void onCreate(Bundle savedInstanceState) {
        sharedWebrogueActivity = this;
        super.onCreate(savedInstanceState);
        // setWindowStyle(true);
    }

    private static WebrogueActivity sharedWebrogueActivity;

    private String getStoragePath() {
        return getFilesDir().getAbsolutePath();
    }

    @Keep
    public static void printBytes(byte[] bytes) {
        System.out.println(new String(bytes, StandardCharsets.UTF_8));
    }
    @Override
    protected String[] getLibraries() {
        return new String[]{ "webrogue" };
    }
}
